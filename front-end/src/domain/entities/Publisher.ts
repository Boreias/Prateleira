import { Book } from "./Book";

export class Publisher {
    constructor(
        private readonly _id: number,
        private _name: string,
        private _site: string,
        private _email: string,
        private _avatar: string,
        private _books: Book[]
    ){
        this._id = _id;
        this._name = _name;
        this._site = _site;
        this._email = _email;
        this._avatar = _avatar;
        this._books = _books;
    }

    get id(){
        return this._id;
    }

    get name(){
        return this._name;
    }

    get site(){
        return this._site;
    }

    get email(){
        return this._email;
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

    setSite(site: string){
        this._site = site;
    }

    setEmail(email: string){
        this._email = email;
    }

    setAvatar(avatar: string){
        this._avatar = avatar;
    }

    setBooks(books: Book[]){
        this._books = books;
    }
}