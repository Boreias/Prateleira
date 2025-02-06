import { Author } from "../../domain/entities/Author";
import { Book } from "../../domain/entities/Book";
import { Gender } from "../../domain/entities/Gender";
import { Publisher } from "../../domain/entities/Publisher";
import { IBookRepository } from "../../domain/irepositories/IBookReposity";
import api from "../api";


export class BookRepository implements IBookRepository {
    private readonly apiUrl = "/books";

    async getAllBooks(): Promise<Book[]> {
        const response = await api.get(this.apiUrl);
        return response.data.map((book: any) => {
            new Book(
                book._id,
                book._title,
                book._subtitle,
                book._authors,
                book._publisher,
                book._series_collection,
                book._volume,
                book._edition,
                book._publication_year,
                book._pages,
                book._language,
                book._isbn,
                book._gender,
                book._synopsis,
                book._cover
            )
        })
    }

    async getBookById(id: number): Promise<Book> {
        const response = await api.get(`${this.apiUrl}/${id}`);
        return new Book(
            response.data._id,
            response.data._title,
            response.data._subtitle,
            response.data._authors,
            response.data._publisher,
            response.data._series_collection,
            response.data._volume,
            response.data._edition,
            response.data._publication_year,
            response.data._pages,
            response.data._language,
            response.data._isbn,
            response.data._gender,
            response.data._synopsis,
            response.data._cover
        );
    }

    async getBookByTitle(title: string): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}?title=${title}`);
        return response.data.map((book: any) => {
            new Book(
                book._id,
                book._title,
                book._subtitle,
                book._authors,
                book._publisher,
                book._series_collection,
                book._volume,
                book._edition,
                book._publication_year,
                book._pages,
                book._language,
                book._isbn,
                book._gender,
                book._synopsis,
                book._cover
            )
        });
    }

    async getBookByAuthor(author: Author): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}?author=${author.id}`);
        return response.data.map((book: any) => {
            new Book(
                book._id,
                book._title,
                book._subtitle,
                book._authors,
                book._publisher,
                book._series_collection,
                book._volume,
                book._edition,
                book._publication_year,
                book._pages,
                book._language,
                book._isbn,
                book._gender,
                book._synopsis,
                book._cover
            )
        });
    }

    async getBookByPublisher(publisher: Publisher): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}?publisher=${publisher.id}`);
        return response.data.map((book: any) => {
            new Book(
                book._id,
                book._title,
                book._subtitle,
                book._authors,
                book._publisher,
                book._series_collection,
                book._volume,
                book._edition,
                book._publication_year,
                book._pages,
                book._language,
                book._isbn,
                book._gender,
                book._synopsis,
                book._cover
            )
        });
    }

    async getBookByGender(gender: Gender): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}?gender=${gender.id}`);
        return response.data.map((book: any) => {
            new Book(
                book._id,
                book._title,
                book._subtitle,
                book._authors,
                book._publisher,
                book._series_collection,
                book._volume,
                book._edition,
                book._publication_year,
                book._pages,
                book._language,
                book._isbn,
                book._gender,
                book._synopsis,
                book._cover
            )
        });
    }

    async saveBook(book: Book): Promise<void> {
        await api.post(this.apiUrl, book);
    }
}