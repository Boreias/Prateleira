import { Book } from "./Book";

export class Publisher {
    constructor(
        private readonly _id: number,
        private _name: string,
        private _books: Book[]
    ){
        this._id = _id;
        this._name = _name;
        this._books = _books;
    }

    get id(){
        return this._id;
    }

    get name(){
        return this._name;
    }

    get books(){
        return this._books;
    }

    setName(name: string){
        this._name = name;
    }

    setBooks(books: Book[]){
        this._books = books;
    }
}