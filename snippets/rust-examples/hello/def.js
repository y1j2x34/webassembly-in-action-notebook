export function reload() {
    location.reload();
}
export class User {
    constructor(id) {
        this.id = id;
    }
    get id() {
        return id;
    }
    set id(i) {
        this.id = i;
    }
    say() {
        console.trace('Hello')
    }
}