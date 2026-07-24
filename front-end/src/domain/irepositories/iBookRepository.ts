import { Book } from "../entities/book"


export interface iBookRepository {
    createBook(bookObject: any): Promise<any>;

    getBookById(id: string): Promise<Book>;

    getBooksByName(book_name: string, skip?: number, page_size?: number): Promise<Book>;

    getBookByIsbn(isbn: string): Promise<Book>;

    getBooksByAuthor(author_id: string, skip?: number, page_size?: number): Promise<Book[]>;

    bestValuatedBooksByAuthor(author_id: string, skip?: number, page_size?: number): Promise<Book[]>;

    getBooksByPublisher(publisher_id: string, skip?: number, page_size?: number): Promise<Book[]>;

    bestValuatedBooksByPublisher(publisher_id: string, skip?: number, page_size?: number): Promise<Book[]>;

    getBooksByGender(gender_id: string, skip?: number, page_size?: number): Promise<Book[]>;

    bestValuatedBooksByGender(gender_id: string, skip?: number, page_size?: number): Promise<Book[]>;

    smalletsBooks(skip?: number, page_size?: number): Promise<Book[]>;

    biggestBooks(skip?: number, page_size?: number): Promise<Book[]>;

    morePopularBooks(skip?: number, page_size?: number): Promise<Book[]>;

    bestValuatedBooks(skip?: number, page_size?: number): Promise<Book[]>;

    alterBook(book_id: string, bookObject: any): Promise<any>;

    deleteBook(book_id: string, user_id: string): Promise<any>;

    clearDeletedBooks(): Promise<any>;
}