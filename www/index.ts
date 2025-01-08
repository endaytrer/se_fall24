import {Game, WasmUserAction, Fisherman, HexCoord, InputResult} from "../pkg"
import "./style.css"
const UPDATE_RADIUS = 4;
const MOVE_RADIUS = 1;
const CELL_HEIGHT = 128;
const TRANSLATE_DELAY = 600
const targetDisplay: HTMLDivElement = document.querySelector("#target");
const hpDisplay: HTMLDivElement = document.querySelector("#health");
const gameMapContainer: HTMLDivElement = document.querySelector("#game");
const binocular: HTMLButtonElement = document.querySelector("#binocular");
const web: HTMLButtonElement = document.querySelector("#web");
const compassAnalog: HTMLDivElement = document.querySelector("#compass-pointer");
const compassDigital: HTMLDivElement = document.querySelector("#compass-digital");
let shiftHold: boolean = false;
let isCapturing: boolean = false;
let game = new Game();

function uniformSampleHexagon(height: number): number[] {
    const scale = height / Math.sqrt(3);
    const triangleIndex = Math.floor(Math.random() * 6);
    const theta = triangleIndex * Math.PI / 3;
    const p0 = Math.random();
    const p1 = Math.random();
    let x = p0 + p1 / 2;
    let y = p1 / 2 * Math.sqrt(3);
    if (p0 + p1 > 1) {
        x /= 2;
        y /= 2;
    }
    return [scale * (x * Math.cos(theta) + y * Math.sin(theta)), scale * (-x * Math.sin(theta) + y * Math.cos(theta))];
}

function handleInputResult(t: InputResult) {
    switch (t) {
        case InputResult.InvalidInput:
            alert("This action is not allowed in harbor!");
            shiftHold = false;
            setNonCapturing();
            break;
        case InputResult.Ok:
            if (!shiftHold) {
                setNonCapturing();
            }
            renderMap(game, gameMapContainer, UPDATE_RADIUS);
            break;
        case InputResult.LevelPassed:
            alert(`Level Passed! score: ${game.get_score()}`)
            shiftHold = false;
            setNonCapturing();
            renderMap(game, gameMapContainer, UPDATE_RADIUS);
            break;
        case InputResult.GamePassed:
            alert(`Congrats! You beat the game! score: ${game.get_score()}`)
            shiftHold = false;
            setNonCapturing();
            game = new Game()
            renderMap(game, gameMapContainer, UPDATE_RADIUS);
            break;
        case InputResult.LevelFailed:
            alert(`Game over! score: ${game.get_score()}`)
            shiftHold = false;
            setNonCapturing();
            game = new Game();
            renderMap(game, gameMapContainer, UPDATE_RADIUS);
            break;
    }
}
function renderMap(game: Game, container: HTMLDivElement, radius: number) {
    const Z_INDEX_ADDER = 100;
    container.innerHTML = "";

    const fisherman = game.get_fisherman();
    const coord = fisherman.get_coord();

    targetDisplay.innerHTML = `Target: ${fisherman.get_captured_marlins()} / ${game.get_target()}`
    hpDisplay.innerHTML = "";
    for (let i = 0; i < fisherman.get_hp(); i++) {
        const heart = document.createElement("span")
        heart.classList.add("heart")
        heart.innerText = "♥";
        hpDisplay.appendChild(heart)
    }
    for (let i = fisherman.get_hp(); i < fisherman.get_initial_hp(); i++) {
        const heart = document.createElement("span")
        heart.classList.add("heart")
        heart.classList.add("empty")
        heart.innerText = "♥";
        hpDisplay.appendChild(heart)
    }
    if (fisherman.get_captured_marlins() >= game.get_target()) {
        targetDisplay.classList.add("fulfilled");
    } else {
        targetDisplay.classList.remove("fulfilled");
    }

    // calculate degree and distance to harbor
    const dist = coord.distance(new HexCoord(0, 0, 0));
    let deg;
    if (dist == 0) {
        deg = 0;
    } else {
        const dir = new HexCoord(-coord.q, -coord.r, -coord.s);
        const zero = dir.r;
        const sixty = dir.q;
        const x = zero + sixty / 2;
        const y = sixty * Math.sqrt(3) / 2;
        const rad = Math.atan2(y, x);
        const rad_normal = rad < 0 ? (rad + Math.PI * 2) : rad;
        deg = rad_normal / Math.PI * 180;
    }

    let orientation;
    if (deg < 22.5 || deg > 337.5) {
        orientation = "N"
    } else if (deg < 67.5) {
        orientation = "NE"
    } else if (deg < 112.5) {
        orientation = "E"
    } else if (deg < 157.5) {
        orientation = "SE"
    } else if (deg < 202.5) {
        orientation = "S"
    } else if (deg < 247.5) {
        orientation = "SW"
    } else if (deg < 292.5) {
        orientation = "W"
    } else {
        orientation = "NW"
    }
    const prev_deg = compassAnalog.style.rotate == "" ? 0 : Number.parseFloat(compassAnalog.style.rotate.substring(0, compassAnalog.style.rotate.length - 3));
    const diff = deg - prev_deg;
    const true_diff = (diff + 180) % 360 - 180;

    compassAnalog.style.rotate = `${prev_deg + true_diff}deg`;
    // cancel absolute degree
    setTimeout(() => {
        compassAnalog.classList.add("no-animation");
        compassAnalog.style.rotate = `${(deg + 180) % 360 - 180}deg`;
        setTimeout(() => {
            compassAnalog.classList.remove("no-animation");
        }, 10);
    }, 200);
    compassDigital.innerHTML = `TO&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|${orientation} ${Math.round(deg)}°<br>HARBOR&nbsp;|${dist} NM`
    for (let dq = -radius; dq <= radius; ++dq) {
        for (let dr = Math.max(-dq, 0) - radius; dr <= Math.min(-dq, 0) + radius; ++dr) {
            const ds = -dq - dr;
            const hexagon = document.createElement("button");
            hexagon.classList.add("hexagon");
            if (Math.abs(dq) + Math.abs(dr) + Math.abs(ds) > MOVE_RADIUS * 2) { // movable
                hexagon.disabled = true;
            } else {
                hexagon.addEventListener("click", (e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    console.log("move")
                    if (isCapturing) {
                        handleInputResult(game.handle_action(WasmUserAction.capture_action(new HexCoord(dq, dr, ds))))
                    } else {
                        if (dq == 0 && dr == 0 && ds == 0) {
                            if (q == 0 && r == 0 && s == 0) {
                                // in harbor, move to nowhere
                                handleInputResult(game.handle_action(WasmUserAction.move_action(new HexCoord(dq, dr, ds))));
                            } else {
                                // don't move nothing, discover
                                handleInputResult(game.handle_action(WasmUserAction.discover_action()));
                            }
                        } else {
                            gameMapContainer.querySelectorAll(".hexagon").forEach((v: HTMLButtonElement) => {
                                v.disabled = true;
                                const tx = Number.parseInt(v.getAttribute("tranx")) - 75 * dq;
                                const ty = Number.parseInt(v.getAttribute("trany")) + 50 * dq + 100 * dr;
                                v.style.transform = `translateX(${tx}%) translateY(${ty}%)`;
                            })
                            setTimeout(() => {
                                handleInputResult(game.handle_action(WasmUserAction.move_action(new HexCoord(dq, dr, ds))));
                            }, TRANSLATE_DELAY);
                        }
                    }
                })
            }
            const q = dq + coord.q;
            const r = dr + coord.r;
            const s = ds + coord.s;
            const absCoord = new HexCoord(q, r, s);
            if (q == 0 && r == 0 && s == 0) {
                // harbor
                let harborElement = document.createElement("div");
                harborElement.classList.add("harbor")
                hexagon.appendChild(harborElement)
            } else {
                const numMarlins = game.get_discovered_marlin_num_at(absCoord);
                const numSharks = game.get_shark_num_at(absCoord);
                for (let i = 0; i < numMarlins; ++i) {
                    let [x, y] = uniformSampleHexagon(CELL_HEIGHT - 30);
                    const ripple = document.createElement("div");
                    ripple.classList.add("marlin-ripple");
                    ripple.style.transform = `translateX(${x}px) translateY(${y}px)`
                    hexagon.appendChild(ripple);
                }
                for (let i = 0; i < numSharks; ++i) {
                    let [x, y] = uniformSampleHexagon(CELL_HEIGHT - 30);
                    const shark = document.createElement("button");
                    shark.classList.add("shark");
                    shark.style.transform = `translateX(${x}px) translateY(${y}px)`
                    shark.addEventListener("click", (e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        console.log("shark")
                        handleInputResult(game.handle_action(WasmUserAction.attack_action(absCoord, i)));
                    })
                    hexagon.appendChild(shark);
                }
            }
            hexagon.setAttribute("coord", `${q},${r},${s}`)
            hexagon.setAttribute("tranx", `${75 * dq}`);
            hexagon.setAttribute("trany", `${-50 * dq - 100 * dr}`);

            hexagon.style.transform = `translateX(${75 * dq}%) translateY(${-50 * dq - 100 * dr}%)`;
            hexagon.style.zIndex = `${dr * 10 + Z_INDEX_ADDER}`;
            container.appendChild(hexagon)
        }
    }
    const fishermanElement = document.createElement("button")
    fishermanElement.classList.add("fisherman");
    fishermanElement.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        if (!isCapturing) {
            const coord = fisherman.get_coord();
            const {q, r, s} = coord;
            if (q == 0 && r == 0 && s == 0) {

                handleInputResult(game.handle_action(WasmUserAction.move_action(new HexCoord(0, 0, 0))))
            } else {

                handleInputResult(game.handle_action(WasmUserAction.discover_action()))
            }
        } else {
            handleInputResult(game.handle_action(WasmUserAction.capture_action(new HexCoord(0, 0, 0))))
        }
    })
    container.appendChild(fishermanElement);
}


renderMap(game, gameMapContainer, UPDATE_RADIUS)

function setCapturing() {
    isCapturing = true;
    web.classList.add("capturing");
}

function setNonCapturing() {
    isCapturing = false;
    web.classList.remove("capturing");
}

binocular.addEventListener("click", (e) => {
    e.preventDefault();
    e.stopPropagation();
    handleInputResult(game.handle_action(WasmUserAction.discover_action()))
})
window.addEventListener("keydown", (e) => {
    if (e.key == "Shift") {
        shiftHold = true;
        setCapturing();
    }
})
window.addEventListener("keyup", (e) => {
    if (e.key == "Shift") {
        shiftHold = false;
        setNonCapturing();
    }
})
web.addEventListener("click", () => {
    if (isCapturing) {
        setNonCapturing();
    } else {
        setCapturing();
    }
})