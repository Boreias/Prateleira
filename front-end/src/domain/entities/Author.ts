import { Book } from "./Book";

export class Author{
    constructor(
        private readonly _id: number,
        private _name: string,
        private _avatar: string,
        private _books: Book[]
    ){
        this._id = _id;
        this._name = _name;
        this._avatar = _avatar;
        this._books = _books;
    }

    get id(){
        return this._id;
    }

    get name(){
        return this._name;
    }

    get avatar(){
        return this._avatar;
    }

    get books(){
        return this._books;
    }

    setName(name: string){
        this._name = name;
    }

    setAvatar(avatar: string){
        this._avatar = avatar;
    }

    setBooks(books: Book[]){
        this._books = books;
    }
}