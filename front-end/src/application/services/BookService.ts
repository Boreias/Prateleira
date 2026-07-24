import type { iBookRepository } from "@/domain/irepositories/iBookRepository";
import { Book } from "@/domain/entities/book";


export class BookService {
    constructor(private bookRepository: iBookRepository) {}

    async createBook(bookObject: any): Promise<any> {
        return await this.bookRepository.createBook(bookObject);
    }

    async getBookById(id: string): Promise<Book> {
        return await this.bookRepository.getBookById(id);
    }

    async getBooksByName(book_name: string, skip?: number, page_size?: number): Promise<Book> {
        return await this.bookRepository.getBooksByName(book_name, skip, page_size);
    }

    async getBookByIsbn(isbn: string): Promise<Book> {
        return await this.bookRepository.getBookByIsbn(isbn);
    }

    async getBooksByAuthor(author_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        return await this.bookRepository.getBooksByAuthor(author_id, skip, page_size);
    }

    async bestValuatedBooksByAuthor(author_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        return await this.bookRepository.bestValuatedBooksByAuthor(author_id, skip, page_size);
    }

    async getBooksByPublisher(publisher_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        return await this.bookRepository.getBooksByPublisher(publisher_id, skip, page_size);
    }

    async bestValuatedBooksByPublisher(publisher_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        return await this.bookRepository.bestValuatedBooksByPublisher(publisher_id, skip, page_size);
    }

    async getBooksByGender(gender_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        return await this.bookRepository.getBooksByGender(gender_id, skip, page_size);
    }

    async bestValuatedBooksByGender(gender_id: string, skip?: number, page_size?: number): Promise<Book[]> {
        return await this.bookRepository.bestValuatedBooksByGender(gender_id, skip, page_size);
    }

    async smalletsBooks(skip?: number, page_size?: number): Promise<Book[]> {
        return await this.bookRepository.smalletsBooks(skip, page_size);
    }

    async biggestBooks(skip?: number, page_size?: number): Promise<Book[]> {
        return await this.bookRepository.biggestBooks(skip, page_size);
    }

    async morePopularBooks(skip?: number, page_size?: number): Promise<Book[]> {
        return await this.bookRepository.morePopularBooks(skip, page_size);
    }

    async bestValuatedBooks(skip?: number, page_size?: number): Promise<Book[]> {
        return await this.bookRepository.bestValuatedBooks(skip, page_size);
    }

    async alterBook(book_id: string, bookObject: any): Promise<any> {
        return await this.bookRepository.alterBook(book_id, bookObject);
    }

    async deleteBook(book_id: string, user_id: string): Promise<any> {
        return await this.bookRepository.deleteBook(book_id, user_id);
    }

    async clearDeletedBooks(): Promise<any> {
        return await this.bookRepository.clearDeletedBooks();
    }
}