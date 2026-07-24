import type { iBookRepository } from "@/domain/irepositories/iBookRepository";
import { Book } from "@/domain/entities/book";
import api from "../api/api";


export class BookRepository implements iBookRepository {
    private readonly apiUrl = "/book";

    async createBook(bookObject: any): Promise<any> {
        const response = await api.post(`${this.apiUrl}create`, bookObject);

        return response;
    }

    async getBookById(id: string): Promise<Book> {
        const response = await api.get(`${this.apiUrl}id?id=${id}`);

        return new Book(
            response.data.id,
            response.data.title,
            response.data.authors,
            response.data.publisher,
            response.data.isbn,
            response.data.genders,
            response.data.subtitle,
            response.data.series_collection,
            response.data.volume,
            response.data.edition,
            response.data.publication_year,
            response.data.pages,
            response.data.language,
            response.data.synopsis,
            response.data.cover
        );
    }

    async getBooksByName(book_name: string, skip?: number, page_size?: number): Promise<Book> {
        const response = await api.get(`${this.apiUrl}name?name=${book_name}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async getBookByIsbn(isbn: string): Promise<Book> {
        const response = await api.get(`${this.apiUrl}isbn?isbn=${isbn}`);

        return new Book(
            response.data.id,
            response.data.title,
            response.data.authors,
            response.data.publisher,
            response.data.isbn,
            response.data.genders,
            response.data.subtitle,
            response.data.series_collection,
            response.data.volume,
            response.data.edition,
            response.data.publication_year,
            response.data.pages,
            response.data.language,
            response.data.synopsis,
            response.data.cover
        );
    }

    async getBooksByAuthor(author_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}author?author_id=${author_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async bestValuatedBooksByAuthor(author_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}best_author_books?author_id=${author_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async getBooksByPublisher(publisher_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}publisher?publisher_id=${publisher_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async bestValuatedBooksByPublisher(publisher_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}best_publisher_books?publisher_id=${publisher_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async getBooksByGender(gender_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}gender?gender_id=${gender_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async bestValuatedBooksByGender(gender_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}best_gender_books?gender_id=${gender_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async smalletsBooks(skip?: number, page_size?: number): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}smallets_books?skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async biggestBooks(skip?: number, page_size?: number): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}biggest_books?skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async morePopularBooks(skip?: number, page_size?: number): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}more_popular?skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async bestValuatedBooks(skip?: number, page_size?: number): Promise<Book[]> {
        const response = await api.get(`${this.apiUrl}best_valuated?skip=${skip}&page_size=${page_size}`);

        return response.data.map((book: any) => new Book(
            book.id,
            book.title,
            book.authors,
            book.publisher,
            book.isbn,
            book.genders,
            book.subtitle,
            book.series_collection,
            book.volume,
            book.edition,
            book.publication_year,
            book.pages,
            book.language,
            book.synopsis,
            book.cover
        ));
    }

    async alterBook(book_id: string, bookObject: any): Promise<any> {
        const response = await api.put(`${this.apiUrl}alter/${book_id}`, bookObject)

        return response;
    }

    async deleteBook(book_id: string, user_id: string): Promise<any> {
        const response = await api.delete(`${this.apiUrl}delete?id=${book_id}&user_id=${user_id}`)

        return response;
    }

    async clearDeletedBooks(): Promise<any> {
        const response = await api.get(`${this.apiUrl}clear_deleted`);

        return response;
    }
}