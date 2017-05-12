import "babel-polyfill";

import Promise from "bluebird";

import fs from "fs";
import path from "path";

import Save from "./save";
import Tnl from "./tnl";
import course from "./course";

module.exports = {
    loadSave: loadSave,
    loadSaveSync: loadSaveSync,
    loadImage: loadImage,
    loadCourse: course.createCourse,
    loadCourseSync: course.createCourseSync
};

async function loadSave(pathToSave) {
    return new Promise((resolve) => {
        pathToSave = path.resolve(pathToSave);
        if (!fs.existsSync(pathToSave)) throw new Error(`No such folder exists:\n${pathToSave}`);
        fs.access(pathToSave, fs.constants.R_OK | fs.constants.W_OK, (err) => {
            if (err) throw new Error("Please close your emulator before executing your script");
        });
        fs.readFile(path.resolve(`${pathToSave}/save.dat`), (err, data) => {
            if (err) throw err;
            resolve(new Save(pathToSave, data));
        });
    });
}

function loadSaveSync(pathToSave) {
    pathToSave = path.resolve(pathToSave);
    if (!fs.existsSync(pathToSave)) throw new Error(`No such folder exists:\n${pathToSave}`);
    fs.access(pathToSave, fs.constants.R_OK | fs.constants.W_OK, (err) => {
        if (err) throw new Error("Please close your emulator before executing your script");
    });
    let data = fs.readFileSync(path.resolve(`${pathToSave}/save.dat`));
    return new Save(pathToSave, data);
}

function loadImage(pathToFile) {
    return new Tnl(pathToFile);
}