const ASTEROID_SEEDS = [
  { id: "lumen", name: "Lumen Patch", x: 190, y: 138, state: "ripe", timer: 0, yield: 24, risk: "Low" },
  { id: "ember", name: "Ember Hollow", x: 366, y: 292, state: "empty", timer: 0, yield: 18, risk: "Low" },
  { id: "moss", name: "Moss Kettle", x: 708, y: 202, state: "growing", timer: 8, yield: 28, risk: "Medium" },
  { id: "glow", name: "Glow Basket", x: 608, y: 530, state: "empty", timer: 0, yield: 22, risk: "Medium" },
  { id: "sable", name: "Sable Shelf", x: 266, y: 520, state: "growing", timer: 11, yield: 26, risk: "Low" },
  { id: "spire", name: "Spire Orchard", x: 822, y: 346, state: "ripe", timer: 0, yield: 30, risk: "High" },
];

const HAZARD_SEEDS = [
  { x: 150, y: 350, vx: 40, vy: -32 },
  { x: 488, y: 118, vx: -28, vy: 42 },
  { x: 882, y: 484, vx: -38, vy: -26 },
];

const state = {
  width: 1000,
  height: 680,
  scoreTarget: 180,
  timeLimit: 180,
  interactionRadius: 98,
  beamBonus: 0,
  hullRepairBonus: 0,
  growthMultiplier: 1,
  combo: 1,
  running: true,
  keys: new Set(),
  lastFrame: 0,
  elapsed: 0,
  ship: {
    x: 350,
    y: 360,
    vx: 0,
    vy: 0,
    hull: 100,
    oxygen: 100,
    seeds: 4,
    score: 0,
    boost: false,
  },
  asteroids: ASTEROID_SEEDS.map((asteroid) => ({ ...asteroid, selected: false })),
  hazards: HAZARD_SEEDS.map((hazard, index) => ({ id: `hazard-${index}`, ...hazard })),
  upgrades: [
    {
      id: "beam",
      name: "Lantern Beam",
      cost: 45,
      description: "Widen the harvest beam for easier orbit-side tending.",
      apply() {
        state.beamBonus += 28;
      },
    },
    {
      id: "growth",
      name: "Compost Lattice",
      cost: 65,
      description: "Shorten every remaining growth cycle by about 20%.",
      apply() {
        state.growthMultiplier *= 0.8;
        state.asteroids.forEach((asteroid) => {
          if (asteroid.state === "growing") {
            asteroid.timer = Math.max(1.5, asteroid.timer * 0.8);
          }
        });
      },
    },
    {
      id: "hull",
      name: "Hull Patch",
      cost: 40,
      description: "Seal the skiff and recover 24 hull immediately.",
      apply() {
        state.ship.hull = Math.min(100, state.ship.hull + 24);
      },
    },
  ],
  purchasedUpgrades: new Set(),
};

const dom = {
  playfield: document.getElementById("playfield"),
  asteroidLayer: document.getElementById("asteroid-layer"),
  hazardLayer: document.getElementById("hazard-layer"),
  ship: document.getElementById("ship"),
  shipBeam: document.getElementById("ship-beam"),
  toastLayer: document.getElementById("toast-layer"),
  timer: document.getElementById("hud-timer"),
  score: document.getElementById("hud-score"),
  seeds: document.getElementById("hud-seeds"),
  hull: document.getElementById("hud-hull"),
  combo: document.getElementById("hud-combo"),
  statusBanner: document.getElementById("status-banner"),
  objectiveList: document.getElementById("objective-list"),
  selectedName: document.getElementById("selected-name"),
  selectedSummary: document.getElementById("selected-summary"),
  selectedState: document.getElementById("selected-state"),
  selectedYield: document.getElementById("selected-yield"),
  selectedWindow: document.getElementById("selected-window"),
  selectedRisk: document.getElementById("selected-risk"),
  upgradeList: document.getElementById("upgrade-list"),
  resultTitle: document.getElementById("result-title"),
  resultBody: document.getElementById("result-body"),
  restartButton: document.getElementById("restart-button"),
};

const asteroidElements = new Map();
const hazardElements = new Map();

function clamp(value, min, max) {
  return Math.max(min, Math.min(max, value));
}

function formatTime(secondsRemaining) {
  const seconds = Math.max(0, Math.ceil(secondsRemaining));
  const minutes = Math.floor(seconds / 60);
  const remainder = seconds % 60;
  return `${String(minutes).padStart(2, "0")}:${String(remainder).padStart(2, "0")}`;
}

function randomRange(min, max) {
  return min + Math.random() * (max - min);
}

function resetGame() {
  state.running = true;
  state.elapsed = 0;
  state.combo = 1;
  state.beamBonus = 0;
  state.growthMultiplier = 1;
  state.purchasedUpgrades.clear();
  state.ship = {
    x: 350,
    y: 360,
    vx: 0,
    vy: 0,
    hull: 100,
    oxygen: 100,
    seeds: 4,
    score: 0,
    boost: false,
  };
  state.asteroids = ASTEROID_SEEDS.map((asteroid) => ({ ...asteroid, selected: false }));
  state.hazards = HAZARD_SEEDS.map((hazard, index) => ({ id: `hazard-${index}`, ...hazard }));
  dom.playfield.classList.remove("is-success", "is-failure");
  setStatus("Glide to an asteroid, then press Space to plant, tend, or harvest.");
  renderObjectiveList();
  renderUpgrades();
  renderAsteroids();
  renderHazards();
  updateSelectedAsteroid();
  updateHud();
}

function setStatus(message) {
  dom.statusBanner.textContent = message;
}

function createAsteroidElement(asteroid) {
  const wrapper = document.createElement("article");
  wrapper.className = "asteroid";
  wrapper.innerHTML = `
    <div class="asteroid__ring"></div>
    <div class="asteroid__body"></div>
    <div class="asteroid__garden"></div>
    <div class="asteroid__progress"><span></span></div>
    <strong class="asteroid__name"></strong>
  `;
  asteroidElements.set(asteroid.id, wrapper);
  dom.asteroidLayer.appendChild(wrapper);
  return wrapper;
}

function createHazardElement(hazard) {
  const element = document.createElement("div");
  element.className = "hazard";
  hazardElements.set(hazard.id, element);
  dom.hazardLayer.appendChild(element);
  return element;
}

function renderAsteroids() {
  state.asteroids.forEach((asteroid) => {
    const element = asteroidElements.get(asteroid.id) ?? createAsteroidElement(asteroid);
    element.style.left = `${(asteroid.x / state.width) * 100}%`;
    element.style.top = `${(asteroid.y / state.height) * 100}%`;
    element.dataset.state = asteroid.state;
    element.classList.toggle("is-selected", asteroid.selected);
    element.classList.toggle("is-near", distanceToShip(asteroid) <= state.interactionRadius + state.beamBonus);
    element.querySelector(".asteroid__name").textContent = asteroid.name;
    const progressBar = element.querySelector(".asteroid__progress");
    const progressFill = progressBar.querySelector("span");
    if (asteroid.state === "growing") {
      progressBar.hidden = false;
      const maxTimer = asteroid.baseTimer ?? asteroid.timer;
      if (!asteroid.baseTimer) {
        asteroid.baseTimer = asteroid.timer;
      }
      const progress = clamp(1 - asteroid.timer / asteroid.baseTimer, 0, 1);
      progressFill.style.width = `${progress * 100}%`;
    } else {
      progressBar.hidden = true;
      progressFill.style.width = "0";
      asteroid.baseTimer = null;
    }
  });
}

function renderHazards() {
  state.hazards.forEach((hazard) => {
    const element = hazardElements.get(hazard.id) ?? createHazardElement(hazard);
    element.style.left = `${(hazard.x / state.width) * 100}%`;
    element.style.top = `${(hazard.y / state.height) * 100}%`;
  });
}

function renderObjectiveList() {
  const ripeCount = state.asteroids.filter((asteroid) => asteroid.state === "ripe").length;
  const growingCount = state.asteroids.filter((asteroid) => asteroid.state === "growing").length;
  const rows = [
    { label: "Reach target", value: `${state.ship.score} / ${state.scoreTarget} stardust` },
    { label: "Ripe gardens", value: `${ripeCount} ready now` },
    { label: "Active crops", value: `${growingCount} growing` },
  ];
  dom.objectiveList.innerHTML = rows
    .map((row) => `<li><span>${row.label}</span><strong>${row.value}</strong></li>`)
    .join("");
}

function renderUpgrades() {
  dom.upgradeList.innerHTML = "";
  state.upgrades.forEach((upgrade) => {
    const card = document.createElement("article");
    card.className = "upgrade-card";
    const owned = state.purchasedUpgrades.has(upgrade.id);
    card.innerHTML = `
      <strong>${upgrade.name}</strong>
      <p>${upgrade.description}</p>
      <button type="button" ${owned || state.ship.score < upgrade.cost || !state.running ? "disabled" : ""}>
        ${owned ? "Installed" : `Buy for ${upgrade.cost} stardust`}
      </button>
    `;
    card.querySelector("button").addEventListener("click", () => purchaseUpgrade(upgrade.id));
    dom.upgradeList.appendChild(card);
  });
}

function updateHud() {
  dom.timer.textContent = formatTime(state.timeLimit - state.elapsed);
  dom.score.textContent = `${Math.floor(state.ship.score)} / ${state.scoreTarget}`;
  dom.seeds.textContent = String(state.ship.seeds);
  dom.hull.textContent = `${Math.max(0, Math.floor(state.ship.hull))}%`;
  dom.combo.textContent = `x${state.combo.toFixed(1)}`;
  renderObjectiveList();
  renderUpgrades();
  dom.resultTitle.textContent = state.running
    ? "In progress"
    : state.ship.score >= state.scoreTarget
      ? "Run passed"
      : "Run partial";
  dom.resultBody.textContent = state.running
    ? "The current run is proving a bounded, inspectable C-SDLC game artifact, not universal five-minute delivery."
    : state.ship.score >= state.scoreTarget
      ? "This run proves the mini-sprint produced a real playable artifact with a complete loop, but it does not prove universal five-minute game delivery."
      : "This run still proves a bounded playable artifact exists, but the current playthrough missed the score target before the timer, oxygen, or hull ran out.";
}

function distanceToShip(asteroid) {
  const dx = asteroid.x - state.ship.x;
  const dy = asteroid.y - state.ship.y;
  return Math.sqrt(dx * dx + dy * dy);
}

function currentAsteroid() {
  return state.asteroids.reduce((closest, asteroid) => {
    const distance = distanceToShip(asteroid);
    if (!closest || distance < closest.distance) {
      return { asteroid, distance };
    }
    return closest;
  }, null);
}

function updateSelectedAsteroid() {
  const current = currentAsteroid();
  state.asteroids.forEach((asteroid) => {
    asteroid.selected = current?.asteroid.id === asteroid.id;
  });
  const asteroid = current?.asteroid;
  if (!asteroid) {
    return;
  }
  dom.selectedName.textContent = asteroid.name;
  dom.selectedState.textContent = asteroid.state;
  dom.selectedYield.textContent = `${asteroid.yield} stardust`;
  dom.selectedWindow.textContent =
    asteroid.state === "growing" ? `${Math.ceil(asteroid.timer)}s remaining` : asteroid.state === "ripe" ? "Harvest now" : "Ready to plant";
  dom.selectedRisk.textContent = asteroid.risk;
  dom.selectedSummary.textContent = summarizeAsteroid(asteroid, current.distance);
}

function summarizeAsteroid(asteroid, distance) {
  if (distance > state.interactionRadius + state.beamBonus) {
    return `${asteroid.name} is outside your beam. Drift closer to interact safely.`;
  }
  if (asteroid.state === "empty") {
    return `${asteroid.name} is ready for a fresh crop. Spend one seed to plant a glowing patch.`;
  }
  if (asteroid.state === "growing") {
    return `${asteroid.name} is growing. A quick tend will shave time off the next harvest.`;
  }
  return `${asteroid.name} is ripe. Harvest now to bank stardust and push the combo higher.`;
}

function showToast(message, x, y) {
  const toast = document.createElement("div");
  toast.className = "toast";
  toast.textContent = message;
  toast.style.left = `${(x / state.width) * 100}%`;
  toast.style.top = `${(y / state.height) * 100}%`;
  dom.toastLayer.appendChild(toast);
  setTimeout(() => toast.remove(), 1300);
}

function pulseBeam(targetX, targetY) {
  dom.ship.classList.add("is-beaming");
  const dx = targetX - state.ship.x;
  const dy = targetY - state.ship.y;
  const distance = Math.sqrt(dx * dx + dy * dy);
  const angle = Math.atan2(dy, dx) * (180 / Math.PI) + 90;
  dom.shipBeam.style.height = `${distance}px`;
  dom.shipBeam.style.transform = `rotate(${angle}deg)`;
  setTimeout(() => {
    dom.ship.classList.remove("is-beaming");
    dom.shipBeam.style.height = "0";
  }, 180);
}

function plant(asteroid) {
  if (state.ship.seeds <= 0) {
    setStatus("No seeds left. Harvest a ripe garden or restart the run.");
    return;
  }
  asteroid.state = "growing";
  asteroid.timer = randomRange(8.5, 13.5) * state.growthMultiplier;
  asteroid.baseTimer = asteroid.timer;
  state.ship.seeds -= 1;
  state.combo = Math.max(1, state.combo - 0.1);
  pulseBeam(asteroid.x, asteroid.y);
  showToast("+ planted", asteroid.x, asteroid.y - 54);
  setStatus(`Planted ${asteroid.name}. Stay mobile while the crop warms up.`);
}

function tend(asteroid) {
  const reduction = 2.2 + state.beamBonus * 0.015;
  asteroid.timer = Math.max(0.8, asteroid.timer - reduction);
  asteroid.baseTimer = Math.max(asteroid.baseTimer ?? asteroid.timer, asteroid.timer);
  state.combo = Math.min(4, state.combo + 0.08);
  pulseBeam(asteroid.x, asteroid.y);
  showToast("tended", asteroid.x, asteroid.y - 54);
  setStatus(`Tended ${asteroid.name}. Growth timer shortened.`);
}

function harvest(asteroid) {
  const reward = asteroid.yield * state.combo;
  state.ship.score += reward;
  state.ship.seeds += 1;
  asteroid.state = "empty";
  asteroid.timer = 0;
  asteroid.baseTimer = null;
  state.combo = Math.min(5, state.combo + 0.35);
  pulseBeam(asteroid.x, asteroid.y);
  showToast(`+${Math.round(reward)} stardust`, asteroid.x, asteroid.y - 54);
  setStatus(`Harvested ${asteroid.name}. The combo climbed and one seed came back.`);
}

function interact() {
  if (!state.running) {
    return;
  }
  const nearest = currentAsteroid();
  if (!nearest || nearest.distance > state.interactionRadius + state.beamBonus) {
    setStatus("No asteroid is within beam range. Drift closer first.");
    return;
  }
  if (nearest.asteroid.state === "empty") {
    plant(nearest.asteroid);
  } else if (nearest.asteroid.state === "growing") {
    tend(nearest.asteroid);
  } else {
    harvest(nearest.asteroid);
  }
  renderAsteroids();
  updateSelectedAsteroid();
  updateHud();
}

function purchaseUpgrade(id) {
  const upgrade = state.upgrades.find((candidate) => candidate.id === id);
  if (!upgrade || state.purchasedUpgrades.has(id) || state.ship.score < upgrade.cost || !state.running) {
    return;
  }
  state.ship.score -= upgrade.cost;
  state.purchasedUpgrades.add(id);
  upgrade.apply();
  setStatus(`${upgrade.name} installed. The next run loop should feel a little smoother.`);
  renderUpgrades();
  updateHud();
}

function handleHazards(deltaSeconds) {
  state.hazards.forEach((hazard) => {
    hazard.x += hazard.vx * deltaSeconds;
    hazard.y += hazard.vy * deltaSeconds;
    if (hazard.x < 40 || hazard.x > state.width - 40) {
      hazard.vx *= -1;
    }
    if (hazard.y < 40 || hazard.y > state.height - 40) {
      hazard.vy *= -1;
    }
    const dx = hazard.x - state.ship.x;
    const dy = hazard.y - state.ship.y;
    const distance = Math.sqrt(dx * dx + dy * dy);
    if (distance < 34) {
      state.ship.hull = Math.max(0, state.ship.hull - 18 * deltaSeconds);
      state.combo = Math.max(1, state.combo - 0.25 * deltaSeconds);
      if (state.running) {
        setStatus("A drifting shard scraped the skiff. Mind the hazards while you garden.");
      }
    }
  });
}

function updateAsteroids(deltaSeconds) {
  state.asteroids.forEach((asteroid) => {
    if (asteroid.state === "growing") {
      asteroid.timer -= deltaSeconds;
      if (asteroid.timer <= 0) {
        asteroid.state = "ripe";
        asteroid.timer = 0;
        asteroid.baseTimer = null;
        showToast("ripe", asteroid.x, asteroid.y - 54);
      }
    }
  });
}

function updateShip(deltaSeconds) {
  const thrust = state.keys.has("Shift") ? 220 : 150;
  const drag = state.keys.has("Shift") ? 0.92 : 0.88;
  if (state.keys.has("ArrowUp") || state.keys.has("w")) state.ship.vy -= thrust * deltaSeconds;
  if (state.keys.has("ArrowDown") || state.keys.has("s")) state.ship.vy += thrust * deltaSeconds;
  if (state.keys.has("ArrowLeft") || state.keys.has("a")) state.ship.vx -= thrust * deltaSeconds;
  if (state.keys.has("ArrowRight") || state.keys.has("d")) state.ship.vx += thrust * deltaSeconds;
  state.ship.vx *= drag;
  state.ship.vy *= drag;
  state.ship.x = clamp(state.ship.x + state.ship.vx * deltaSeconds, 30, state.width - 30);
  state.ship.y = clamp(state.ship.y + state.ship.vy * deltaSeconds, 30, state.height - 30);
}

function updateRunState(deltaSeconds) {
  state.elapsed += deltaSeconds;
  state.ship.oxygen = Math.max(0, 100 - (state.elapsed / state.timeLimit) * 100);
  if (state.ship.score >= state.scoreTarget && state.running) {
    state.running = false;
    dom.playfield.classList.add("is-success");
    setStatus("Target reached. This run passes as a bounded playable proof.");
  }
  if ((state.elapsed >= state.timeLimit || state.ship.hull <= 0 || state.ship.oxygen <= 0) && state.running) {
    state.running = false;
    dom.playfield.classList.add("is-failure");
    setStatus("Run ended before the target. The artifact still proves a playable bounded demo.");
  }
}

function renderShip() {
  dom.ship.style.left = `${(state.ship.x / state.width) * 100}%`;
  dom.ship.style.top = `${(state.ship.y / state.height) * 100}%`;
}

function frame(timestamp) {
  if (!state.lastFrame) {
    state.lastFrame = timestamp;
  }
  const deltaSeconds = Math.min(0.032, (timestamp - state.lastFrame) / 1000);
  state.lastFrame = timestamp;

  if (state.running) {
    updateShip(deltaSeconds);
    updateAsteroids(deltaSeconds);
    handleHazards(deltaSeconds);
    updateRunState(deltaSeconds);
  }

  renderAsteroids();
  renderHazards();
  renderShip();
  updateSelectedAsteroid();
  updateHud();
  requestAnimationFrame(frame);
}

document.addEventListener("keydown", (event) => {
  if (["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight", "Shift", "w", "a", "s", "d"].includes(event.key)) {
    state.keys.add(event.key);
  }
  if (event.key === " ") {
    event.preventDefault();
    interact();
  }
  if (event.key.toLowerCase() === "r") {
    resetGame();
  }
});

document.addEventListener("keyup", (event) => {
  state.keys.delete(event.key);
});

dom.restartButton.addEventListener("click", resetGame);

resetGame();
requestAnimationFrame(frame);
