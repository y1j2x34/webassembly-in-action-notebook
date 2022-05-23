export function reload() {
    console.log('reload method')
}
export class User {
    constructor(id) {
        this._id = id;
    }
    get id() {
        return this._id;
    }
    set id(i) {
        this._id = i;
    }
    say() {
        console.trace('Hello')
    }
}