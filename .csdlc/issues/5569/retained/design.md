# Design: repair #5547 terminal plan truth

Use a separately claimed typed authority record to advance only the four
already-proven #5547 SPP steps from pending to completed. Each operation must
CAS both the local terminal record and retained receipt. No execution,
validation, review, publication, merge, or terminal evidence is invented; the
repair only aligns plan status with evidence already retained in #5547.
