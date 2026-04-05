import init, { Renderer } from "./pkg/weather.js";

async function main() {
    const loader = document.getElementById("loader");
    const container = document.querySelector(".container");

    try {
        await init();
        const canvas = document.getElementById("weather-canvas");
        const renderer = new Renderer(canvas);

        console.log("Loading assets...");
        const sprites = await loadSprites();
        
        const { lat, lon, city } = await getPosition();
        document.getElementById("location-label").innerText = `${lat.toFixed(2)}, ${lon.toFixed(2)}`;
        document.getElementById("city-name").innerText = city;

        console.log("Fetching weather...");
        const forecast = await fetchWeather(lat, lon);
        
        applyTheme(forecast.current);
        updateDashboard(forecast.current);

        renderer.draw_landscape(forecast, sprites);

        loader.style.opacity = 0;
        setTimeout(() => {
            loader.style.display = "none";
        }, 800);

    } catch (e) {
        console.error("Atmosphere Synchronization Failure:", e);
        loader.innerHTML = `<p style='color: #ff4d4d'>Failed to sync atmosphere: ${e.message}</p>`;
    }
}

function applyTheme(weather) {
    const body = document.body;
    body.classList.remove("sunny", "rainy", "cloudy", "night");

    const hour = new Date().getHours();
    const isNight = hour < 6 || hour > 20;

    if (isNight) {
        body.classList.add("night");
    } else if (weather.rain > 0.1) {
        body.classList.add("rainy");
    } else if (weather.clouds > 50) {
        body.classList.add("cloudy");
    } else {
        body.classList.add("sunny");
    }
}

function updateDashboard(weather) {
    document.getElementById("temp-val").innerText = weather.temperature.toFixed(1);
    document.getElementById("press-val").innerText = Math.round(weather.pressure);
    document.getElementById("wind-val").innerText = weather.wind_speed.toFixed(1);
    document.getElementById("cloud-val").innerText = Math.round(weather.clouds);
}

async function loadSprites() {
    const spriteNames = [
        "cloud_02", "cloud_03", "cloud_05", "cloud_10", "cloud_30", "cloud_50",
        "digit_00", "digit_01", "digit_02", "digit_03", "digit_04", "digit_05", "digit_06", "digit_07", "digit_08", "digit_09", "digit_10", "digit_11", "digit_12",
        "east_00", "east_01", "east_02", "east_03",
        "flower_00", "flower_01",
        "house_00", "house_01", "house_02",
        "moon_00", "moon_01",
        "palm_00", "palm_01", "palm_02", "palm_03",
        "pine_00", "pine_01", "pine_02", "pine_03",
        "sun_00", "tree_00", "tree_01", "tree_02", "tree_03"
    ];

    const spriteMap = new Map();
    const promises = spriteNames.map(name => {
        return new Promise((resolve) => {
            const img = new Image();
            img.onload = () => {
                const processed = stripMagenta(img);
                spriteMap.set(name, processed);
                resolve();
            };
            img.onerror = () => { resolve(); };
            img.src = `assets/${name}.png`;
        });
    });

    await Promise.all(promises);
    return spriteMap;
}

function stripMagenta(img) {
    const canvas = document.createElement("canvas");
    canvas.width = img.width;
    canvas.height = img.height;
    const ctx = canvas.getContext("2d");
    ctx.drawImage(img, 0, 0);
    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    const data = imageData.data;

    for (let i = 0; i < data.length; i += 4) {
        if (data[i] > 200 && data[i + 1] < 50 && data[i + 2] > 200) {
            data[i + 3] = 0;
        }
    }
    ctx.putImageData(imageData, 0, 0);
    return canvas;
}

async function getPosition() {
    return new Promise((resolve) => {
        if ("geolocation" in navigator) {
            navigator.geolocation.getCurrentPosition(
                async (pos) => { resolve({ lat: pos.coords.latitude, lon: pos.coords.longitude, city: "Local Position" }); },
                () => { resolve({ lat: 52.52, lon: 13.41, city: "Berlin (Default)" }); }
            );
        } else {
            resolve({ lat: 52.52, lon: 13.41, city: "Berlin (Default)" });
        }
    });
}

async function fetchWeather(lat, lon) {
    const url = `https://api.open-meteo.com/v1/forecast?latitude=${lat}&longitude=${lon}&hourly=temperature_2m,precipitation,cloud_cover,wind_speed_10m,wind_direction_10m,surface_pressure&current_weather=true&timeout=10000`;
    const response = await fetch(url);
    const data = await response.json();

    const hourly = [];
    for (let i = 0; i < 24; i++) {
        hourly.push({
            temperature: data.hourly.temperature_2m[i],
            rain: data.hourly.precipitation[i],
            snow: 0,
            clouds: data.hourly.cloud_cover[i],
            wind_speed: data.hourly.wind_speed_10m[i],
            wind_deg: data.hourly.wind_direction_10m[i],
            pressure: data.hourly.surface_pressure[i],
            time: Math.floor(new Date(data.hourly.time[i]).getTime() / 1000)
        });
    }

    const temperatures = data.hourly.temperature_2m.slice(0, 24);
    
    return {
        hourly: hourly,
        current: {
            temperature: data.current_weather.temperature,
            rain: data.hourly.precipitation[0],
            snow: 0,
            clouds: data.hourly.cloud_cover[0],
            wind_speed: data.current_weather.windspeed,
            wind_deg: data.current_weather.winddirection,
            pressure: data.hourly.surface_pressure[0],
            time: Math.floor(Date.now() / 1000)
        },
        tmin: Math.min(...temperatures),
        tmax: Math.max(...temperatures)
    };
}

main();
