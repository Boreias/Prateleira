import { Author } from "./Author";
import { Gender } from "./Gender";
import { Publisher } from "./Publisher";

export class Book {
    constructor(
        private readonly _id: number,
        private _title: string,
        private _subtitle: string,
        private _authors: Author[],
        private _publisher: Publisher,
        private _series_collection: number,
        private _volume: number,
        private _edition: number,
        private _publication_year: number,
        private _pages: number,
        private _language: string,
        private _isbn: string,
        private _gender: Gender[],
        private _synopsis: string,
        private _cover: string
    ){
        this._id = _id;
        this._title = _title;
        this._subtitle = _subtitle;
        this._authors = _authors;
        this._publisher = _publisher;
        this._series_collection = _series_collection;
        this._volume = _volume;
        this._edition = _edition;
        this._publication_year = _publication_year;
        this._pages = _pages;
        this._language = _language;
        this._isbn = _isbn;
        this._gender = _gender;
        this._synopsis = _synopsis;
        this._cover = _cover;
    }

    get id(){
        return this._id;
    }

    get title(){
        return this._title;
    }

    get subtitle(){
        return this._subtitle;
    }

    get authors(){
        return this._authors;
    }

    get publisher(){
        return this._publisher;
    }

    get series_collection(){
        return this._series_collection;
    }

    get volume(){
        return this._volume;
    }

    get edition(){
        return this._edition;
    }

    get publication_year(){
        return this._publication_year;
    }

    get pages(){
        return this._pages;
    }

    get language(){
        return this._language;
    }

    get isbn(){
        return this._isbn;
    }

    get gender(){
        return this._gender;
    }

    get synopsis(){
        return this._synopsis;
    }

    get cover(){
        return this._cover;
    }

    setTitle(title: string){
        this._title = title;
    }

    setSubtitle(subtitle: string){
        this._subtitle = subtitle;
    }

    setAuthors(authors: Author[]){
        this._authors = authors;
    }

    setPublisher(publisher: Publisher){
        this._publisher = publisher;
    }

    setSeriesCollection(series_collection: number){
        this._series_collection = series_collection;
    }

    setVolume(volume: number){
        this._volume = volume;
    }

    setEdition(edition: number){
        this._edition = edition;
    }

    setPublicationYear(publication_year: number){
        this._publication_year = publication_year;
    }

    setPages(pages: number){
        this._pages = pages;
    }

    setLanguage(language: string){
        this._language = language;
    }

    setIsbn(isbn: string){
        this._isbn = isbn;
    }

    setGender(gender: Gender[]){
        this._gender = gender;
    }

    setSynopsis(synopsis: string){
        this._synopsis = synopsis;
    }

    setCover(cover: string){
        this._cover = cover;
    }
}