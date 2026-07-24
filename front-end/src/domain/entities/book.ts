import { Author } from "./author";
import { Publisher } from "./publisher";
import { Gender } from "./gender";


export class Book {
    constructor(
        private readonly _id: string,
        private _title: string,
        private _authors: Author[],
        private _publisher: Publisher,
        private _isbn: string,
        private _genders: Gender[],
        private _subtitle?: string,
        private _series_collection?: number,
        private _volume?: number,
        private _edition?: number,
        private _publication_year?: number,
        private _pages?: number,
        private _language?: string,
        private _synopsis?: string,
        private _cover?: string
    ) {
        this._id = _id,
        this._title = _title,
        this._authors = _authors,
        this._publisher = _publisher,
        this._isbn = _isbn,
        this._genders = _genders,
        this._subtitle = _subtitle,
        this._series_collection = _series_collection,
        this._volume = _volume,
        this._edition = _edition,
        this._publication_year = _publication_year,
        this._pages = _pages,
        this._language = _language,
        this._synopsis = _synopsis,
        this._cover = _cover
    }

    get id() {
        return this._id;
    }

    get title () {
        return this._title;
    }

    setTitle(title: string) {
        this._title = title;
    }
    
    get subtitle() {
        return this._subtitle;
    }

    setSubtitle(subtitle: string) {
        return this._subtitle;
    }

    get authors() {
        return this._authors;
    }

    setAuthors(authors: Author[]) {
        this._authors = authors;
    }

    get publisher() {
        return this._publisher;
    }

    setPublisher(publisher: Publisher) {
        return this._publisher;
    }

    get series_collection() {
        return this._series_collection;
    }

    setSeriesCollection(series_collection: number) {
        this._series_collection = series_collection;
    }

    get volume() {
        return this._volume;
    }

    setVolume(volume: number) {
        this._volume = volume;
    }

    get edition() {
        return this._edition;
    }

    setEdition(edition: number) {
        return this._edition;
    }

    get publication_year() {
        return this._publication_year;
    }

    setPublicationYear(publication_year: number) {
        this._publication_year = publication_year;
    }

    get pages() {
        return this._pages;
    }

    setPages(pages: number) {
        this._pages = pages;
    }

    get language() {
        return this._language;
    }

    setLanguage(language: string) {
        this._language = language;
    }

    get isbn() {
        return this._isbn;
    }

    setIsbn(isbn: string) {
        this._isbn = isbn;
    }

    get genders() {
        return this._genders;
    }

    setGenders(genders: Gender[]) {
        this._genders = genders;
    }

    get synopsis() {
        return this._synopsis;
    }

    setSynopsis(synopsis: string) {
        this._synopsis = synopsis;
    }

    get cover() {
        return this._cover;
    }

    setCover(cover: string) {
        this._cover = cover;
    }
}