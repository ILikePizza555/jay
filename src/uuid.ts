import { Buffer } from "buffer";
import uuid from "uuid";

/**
 * Simple wrapper around a UUID buffer.
 */
export class UUID {
    private _buffer: Buffer;

    constructor();
    constructor(uuidBuffer?: Buffer) {
        if (!uuidBuffer) {
            this._buffer = Buffer.alloc(128);
            uuid.v1({}, this._buffer);
        } else {
            this._buffer = uuidBuffer;
        }
    }

    get asBuffer(): Buffer {
        return this._buffer;
    }

    get asString(): string {
        return uuid.stringify(this._buffer);
    }
}