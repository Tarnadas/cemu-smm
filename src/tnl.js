import Promise   from 'bluebird'
import * as jimp from 'jimp'
import crc32     from 'buffer-crc32'

import * as fs   from 'fs'
import * as path from 'path'

const TNL_SIZE = 0xC800;
const TNL_JPEG_MAX_SIZE = 0xC7F8;
const TNL_DIMENSION = [
    [ 720, 81 ],
    [ 320, 240 ]
];
const TNL_ASPECT_RATIO = [
    TNL_DIMENSION[0][0] / TNL_DIMENSION[0][1],
    TNL_DIMENSION[1][0] / TNL_DIMENSION[1][1]
];
const TNL_ASPECT_RATIO_THRESHOLD = [ 3.5, 0.3 ];

const IMAGE_BUFFER = Buffer.concat([
    Buffer.from(
        `42 4D 0A 1C 01 00 00 00 00 00 8A 00 00 00 7C 00 
        00 00 D8 00 00 00 A8 00 00 00 01 00 10 00 03 00 
        00 00 80 1B 01 00 13 0B 00 00 13 0B 00 00 00 00 
        00 00 00 00 00 00 00 F8 00 00 E0 07 00 00 1F 00 
        00 00 00 00 00 00 42 47 52 73 00 00 00 00 00 00 
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
        00 00 00 00 00 00 00 00 00 00 02 00 00 00 00 00 
        00 00 00 00 00 00 00 00 00 00`.replace(/\s+/g, ''), 'hex'
    )],
    0x11C0A
);
const IMAGE_BUFFER_WIDE = Buffer.concat([
    Buffer.from(
        `42 4D 8A 3C 00 00 00 00 00 00 8A 00 00 00 7C 00 
        00 00 F0 00 00 00 20 00 00 00 01 00 10 00 03 00 
        00 00 00 3C 00 00 13 0B 00 00 13 0B 00 00 00 00 
        00 00 00 00 00 00 00 F8 00 00 E0 07 00 00 1F 00 
        00 00 00 00 00 00 42 47 52 73 00 00 00 00 00 00 
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
        00 00 00 00 00 00 00 00 00 00 02 00 00 00 00 00 
        00 00 00 00 00 00 00 00 00 00`.replace(/\s+/g, ''), 'hex'
    )],
    0x3C8A
);

export class Image {

    constructor (data, dataWide) {
        if (data instanceof Buffer) {
            this.data = data;
            this.dataWide = dataWide;
        } else {
            this.pathToFile = path.resolve(data);
            if (!fs.existsSync(this.pathToFile)) throw new Error(`No such file exists:\n${this.pathToFile}`);
        }
    }

    async readFile () {
        this.data = await new Promise((resolve) => {
            fs.readFile(this.pathToFile, (err, data) => {
                if (err) throw err;
                resolve(data);
            });
        });
        return this;
    }

    readFileSync () {
        this.data = fs.readFileSync(this.pathToFile);
        return this;
    }

    async from3DS () {

        const mortonEnc = (x, y, w, offs = 0) => {
            let i = (x & 7) | ((y & 7) << 8);
            i = (i ^ (i << 2)) & 0x1313;
            i = (i ^ (i << 1)) & 0x1515;
            i = (i | (i >>> 7)) & 0x3F;
            let offset = (x & ~7) * 8;
            let m = i + offset;
            let o = (y & ~7) * w;
            return (m + o) * 2 + offs;
        };

        // no idea how to create an empty image with jimp, so I created one in Gimp and wrote its header to buffer
        let image = await jimp.read(Buffer.from(IMAGE_BUFFER));
        for (let y = 0; y < 216; y++) {
            for (let x = 0; x < 168; x++) {
                let s = mortonEnc(x, y, 168);
                const rgb565 = this.data.readUInt16LE(s);
                const hex = jimp.rgbaToInt(((rgb565 & 0xF800) >>> 11) << 3, ((rgb565 & 0x7E0) >>> 5) << 2, (rgb565 & 0x1F) << 3, 0xFF);
                image.setPixelColor(hex, y, 167 - x);
            }
        }
        let img = await new Promise(resolve => {
            image.quality(100).autocrop().getBuffer(jimp.MIME_JPEG, (err, img) => {
                resolve(img);
            });
        });

        image = await jimp.read(Buffer.from(IMAGE_BUFFER_WIDE));
        for (let y = 0; y < 240; y++) {
            for (let x = 0; x < 32; x++) {
                let s = mortonEnc(x, y, 32, 0x11B80);
                let rgb565 = this.data.readUInt16LE(s);
                let hex = jimp.rgbaToInt(((rgb565 & 0xF800) >>> 11) << 3, ((rgb565 & 0x7E0) >>> 5) << 2, (rgb565 & 0x1F) << 3, 0xFF);
                image.setPixelColor(hex, y, 31 - x);
            }
        }
        let imgWide = await new Promise(resolve => {
            image.quality(100).autocrop().getBuffer(jimp.MIME_JPEG, (err, img) => {
                resolve(img);
            });
        });
        return [imgWide, img];

    }

    async to3DS () {

        const mortonEnc = (x, y, w, offs = 0) => {
            let i = (x & 7) | ((y & 7) << 8);
            i = (i ^ (i << 2)) & 0x1313;
            i = (i ^ (i << 1)) & 0x1515;
            i = (i | (i >>> 7)) & 0x3F;
            let offset = (x & ~7) * 8;
            let m = i + offset;
            let o = (y & ~7) * w;
            return (m + o) * 2 + offs;
        };

        let res = Buffer.alloc(0x157C0);
        let data = (await jimp.read(this.data)).contain(216, 168);
        let dataWide = (await jimp.read(this.dataWide)).contain(240, 32);
        for (let y = 0; y < 216; y++) {
            for (let x = 0; x < 168; x++) {
                let s = mortonEnc(x, y, 168);
                const color = data.getPixelColor(y, 176 - x);
                const rgba565 = (((color & 0xFF000000) >>> 27) << 11) | (((color & 0x00FF0000) >>> 18) << 5) | ((color & 0x0000FF00) >>> 11);
                res.writeUInt16LE(rgba565, s);
            }
        }
        for (let y = 0; y < 240; y++) {
            for (let x = 0; x < 32; x++) {
                let s = mortonEnc(x, y, 32, 0x11B80);
                const color = dataWide.getPixelColor(y, 31 - x);
                const rgba565 = (((color & 0xFF000000) >>> 27) << 11) | (((color & 0x00FF0000) >>> 18) << 5) | ((color & 0x0000FF00) >>> 11);
                res.writeUInt16LE(rgba565, s);
            }
        }
        return res;

    }

}

/**
 * A TNL file
 * @class Tnl
 */
export class Tnl extends Image {

    constructor (data) {
        super(data);
    }

    /**
     * Convert to JPEG
     * @function toJpeg
     * @memberOf Tnl
     * @instance
     * @returns {Promise.<Buffer|ArrayBuffer>}
     */
    async toJpeg () {

        if (!this.data) {
            await this.readFile();
        }
        let length = this.data.readUInt32BE(4);
        return this.data.slice(8, 8 + length);

    }

    /**
     * Synchronous version of {@link Tnl.toJpeg}
     * @function toJpegSync
     * @memberOf Tnl
     * @instance
     * @returns {Buffer|ArrayBuffer}
     */
    toJpegSync () {

        if (!this.data) {
            this.data = fs.readFileSync(this.pathToFile);
        }
        let length = this.data.readUInt32BE(4);
        return this.data.slice(8, 8 + length);

    }

    /**
     * Check if TNL thumbnail is broken and needs fix
     * @function isBroken
     * @memberOf Tnl
     * @instance
     * @returns {Promise.<boolean>}
     */
    async isBroken () {

        if (!this.data) {
            await this.readFile();
        }
        let length = this.data.readUInt32BE(4);
        let jpeg = this.data.slice(8, 8 + length);
        let count = 0;
        try {
            for (let i = 0; i < jpeg.length; i+=4) {
                if (jpeg.readUInt32BE(i) === 0xA2800A28) {
                    count++;
                }
            }
        } catch (err) {}
        return (count*4 / jpeg.length) > 0.5;

    }

}

/**
 * A JPEG file
 * @class Jpeg
 */
export class Jpeg extends Image {

    constructor (data) {
        super(data);
    }

    /**
     * Convert to TNL
     * @function toTnl
     * @memberOf Jpeg
     * @instance
     * @returns {Promise.<Buffer|ArrayBuffer>}
     */
    async toTnl (isWide, doClip = false) {

        return new Promise(async (resolve, reject) => {

            let sizeOK = false;
            if (!this.data) {
                await this.readFile();
            }
            if (this.data.length <= TNL_JPEG_MAX_SIZE) {
                sizeOK = true;
            }

            let image = await jimp.read(this.data);
            image.autocrop();
            let skipPreprocessing = false;
            if (sizeOK && ((isWide || isWide == null) && (image.bitmap.width === TNL_DIMENSION[0][0] && image.bitmap.height === TNL_DIMENSION[0][1]) ||
                ((!isWide || isWide == null) && image.bitmap.width === TNL_DIMENSION[1][0] && image.bitmap.height === TNL_DIMENSION[1][1]))) {
                skipPreprocessing = true;
            }

            // image pre-processing
            if (!skipPreprocessing) {
                if (isWide == null) {
                    let aspectRatio = image.bitmap.width / image.bitmap.height;
                    if (aspectRatio > TNL_ASPECT_RATIO[0] - TNL_ASPECT_RATIO_THRESHOLD[0] && aspectRatio < TNL_ASPECT_RATIO[0] + TNL_ASPECT_RATIO_THRESHOLD[0]) {
                        isWide = true;
                    } else if (aspectRatio > TNL_ASPECT_RATIO[1] - TNL_ASPECT_RATIO_THRESHOLD[1] && aspectRatio < TNL_ASPECT_RATIO[1] + TNL_ASPECT_RATIO_THRESHOLD[1]) {
                        isWide = false;
                    }
                    if (isWide == null) {
                        isWide = TNL_ASPECT_RATIO[0] - TNL_ASPECT_RATIO_THRESHOLD[0] - aspectRatio <= TNL_ASPECT_RATIO[1] + TNL_ASPECT_RATIO_THRESHOLD[1] + aspectRatio;
                    }
                }

                if (isWide) {
                    if (doClip) {
                        image.cover(TNL_DIMENSION[0][0], TNL_DIMENSION[0][1]);
                    } else {
                        let aspectRatio = image.bitmap.width / image.bitmap.height;
                        let width = aspectRatio < TNL_ASPECT_RATIO[0] ? (
                            aspectRatio * TNL_DIMENSION[0][0] / TNL_ASPECT_RATIO[0]
                        ) : (
                            TNL_DIMENSION[0][0]
                        );
                        let height = aspectRatio > TNL_ASPECT_RATIO[0] ? (
                            TNL_ASPECT_RATIO[0] * TNL_DIMENSION[0][1] / aspectRatio
                        ) : (
                            TNL_DIMENSION[0][1]
                        );
                        image.contain(width, height);
                    }
                } else {
                    if (doClip) {
                        image.cover(TNL_DIMENSION[1][0], TNL_DIMENSION[1][1]);
                    } else {
                        let aspectRatio = image.bitmap.width / image.bitmap.height;
                        let width = aspectRatio < TNL_ASPECT_RATIO[1] ? (
                            aspectRatio * TNL_DIMENSION[1][0] / TNL_ASPECT_RATIO[1]
                        ) : (
                            TNL_DIMENSION[1][0]
                        );
                        let height = aspectRatio > TNL_ASPECT_RATIO[1] ? (
                            TNL_ASPECT_RATIO[1] * TNL_DIMENSION[1][1] / aspectRatio
                        ) : (
                            TNL_DIMENSION[1][1]
                        );
                        image.contain(width, height);
                    }
                }

                let quality = 95;
                this.data = await new Promise((resolve) => {
                    image.quality(quality);
                    image.getBuffer(jimp.MIME_JPEG, (err, buffer) => { resolve(buffer); });
                });

                // lower quality until it fits
                while (this.data.length > TNL_JPEG_MAX_SIZE) {
                    quality -= 5;
                    if (quality < 0) {
                        reject("File could not be transformed into jpeg with lowest quality setting.");
                    }
                    this.data = await new Promise((resolve) => {
                        image.quality(quality);
                        image.getBuffer(jimp.MIME_JPEG, (err, buffer) => { resolve(buffer); });
                    });
                }
            }

            // wrap TNL data around JPEG
            let length = Buffer.alloc(4);
            length.writeUInt32BE(this.data.length, 0);

            let padding = Buffer.alloc(0xC800 - this.data.length - 8);

            let fileWithoutCrc = Buffer.concat([length, this.data, padding], 0xC800 - 4);

            let crcBuffer = Buffer.alloc(4);
            crcBuffer.writeUInt32BE(crc32.unsigned(fileWithoutCrc), 0);

            let tnl = Buffer.concat([crcBuffer, fileWithoutCrc], TNL_SIZE);
            resolve(tnl);

        });

    }

    /**
     * Check if JPEG thumbnail is broken and needs fix
     * @function isBroken
     * @memberOf Jpeg
     * @instance
     * @returns {Promise.<boolean>}
     */
    async isBroken () {

        if (!this.data) {
            await this.readFile();
        }
        let count = 0;
        try {
            for (let i = 0; i < this.data.length; i+=4) {
                if (this.data.readUInt32BE(i) === 0xA2800A28) {
                    count++;
                }
            }
        } catch (err) {}
        return (count*4 / this.data.length) > 0.5;

    }

}