import { Book } from "./book";


export class Author {
    constructor(
        private readonly _id: string,
        private _name: string,
        private _avatar: string,
        private _books: Book[]
    ) {
        this._id = _id;
        this._name = _name;
        this._avatar = _avatar;
        this._books = _books;
    }
}