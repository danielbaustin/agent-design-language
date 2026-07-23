use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{ErrorCode, Limits, Record, RecordError, Result};

const NULL: u8 = 0x00;
const FALSE: u8 = 0x01;
const TRUE: u8 = 0x02;
const UNSIGNED: u8 = 0x03;
const SIGNED: u8 = 0x04;
const STRING: u8 = 0x05;
const ARRAY: u8 = 0x07;
const OBJECT: u8 = 0x08;
const CANONICAL_DOMAIN: &[u8] = b"ADL-RECORD-CANONICAL\0\x00\x01";

pub fn canonical_bytes(record: &Record, limits: &Limits) -> Result<Vec<u8>> {
    record.validate(limits)?;
    let value = serde_json::to_value(record)
        .map_err(|_| RecordError::new(ErrorCode::Canonical, "record serialization failed"))?;
    let mut output = CANONICAL_DOMAIN.to_vec();
    encode(&value, &mut output, limits, 0, &mut 0)?;
    if output.len() > limits.max_payload_bytes {
        return Err(RecordError::new(
            ErrorCode::Bounds,
            "canonical payload bound exceeded",
        ));
    }
    Ok(output)
}

pub fn payload_digest(record: &Record, limits: &Limits) -> Result<[u8; 32]> {
    Ok(Sha256::digest(canonical_bytes(record, limits)?).into())
}

fn encode(
    value: &Value,
    output: &mut Vec<u8>,
    limits: &Limits,
    depth: usize,
    members: &mut usize,
) -> Result<()> {
    if depth > limits.max_json_depth {
        return Err(RecordError::new(
            ErrorCode::Bounds,
            "canonical depth exceeded",
        ));
    }
    match value {
        Value::Null => output.push(NULL),
        Value::Bool(false) => output.push(FALSE),
        Value::Bool(true) => output.push(TRUE),
        Value::Number(number) => {
            if let Some(unsigned) = number.as_u64() {
                output.push(UNSIGNED);
                output.extend_from_slice(&unsigned.to_be_bytes());
            } else if let Some(signed) = number.as_i64() {
                output.push(SIGNED);
                output.extend_from_slice(&signed.to_be_bytes());
            } else {
                return Err(RecordError::new(
                    ErrorCode::Canonical,
                    "floating point is forbidden",
                ));
            }
        }
        Value::String(string) => {
            output.push(STRING);
            write_bytes(output, string.as_bytes())?;
        }
        Value::Array(values) => {
            output.push(ARRAY);
            write_len(output, values.len())?;
            charge(members, values.len(), limits)?;
            for item in values {
                encode(item, output, limits, depth + 1, members)?;
                check_output(output, limits)?;
            }
        }
        Value::Object(values) => {
            output.push(OBJECT);
            write_len(output, values.len())?;
            charge(members, values.len(), limits)?;
            let mut entries: Vec<_> = values.iter().collect();
            entries.sort_by(|(left, _), (right, _)| left.as_bytes().cmp(right.as_bytes()));
            for (key, item) in entries {
                write_bytes(output, key.as_bytes())?;
                encode(item, output, limits, depth + 1, members)?;
                check_output(output, limits)?;
            }
        }
    }
    check_output(output, limits)
}

fn charge(total: &mut usize, amount: usize, limits: &Limits) -> Result<()> {
    *total = total
        .checked_add(amount)
        .ok_or_else(|| RecordError::new(ErrorCode::Bounds, "member count overflow"))?;
    if *total > limits.max_json_members {
        return Err(RecordError::new(ErrorCode::Bounds, "member bound exceeded"));
    }
    Ok(())
}

fn write_len(output: &mut Vec<u8>, length: usize) -> Result<()> {
    let length = u32::try_from(length)
        .map_err(|_| RecordError::new(ErrorCode::Bounds, "length exceeds u32"))?;
    output.extend_from_slice(&length.to_be_bytes());
    Ok(())
}

fn write_bytes(output: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    write_len(output, bytes.len())?;
    output.extend_from_slice(bytes);
    Ok(())
}

fn check_output(output: &[u8], limits: &Limits) -> Result<()> {
    if output.len() > limits.max_payload_bytes {
        return Err(RecordError::new(
            ErrorCode::Bounds,
            "canonical payload bound exceeded",
        ));
    }
    Ok(())
}
