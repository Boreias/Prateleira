import { Book } from "../entities/Book";
import { Publisher } from "../entities/Publisher";
import { Author } from "../entities/Author";
import { Gender } from "../entities/Gender";


export interface IBookRepository {
    getAllBooks(): Promise<Book[]>;
    getBookById(id: number): Promise<Book>;
    getBookByTitle(title: string): Promise<Book[]>;
    getBookByAuthor(author: Author): Promise<Book[]>;
    getBookByPublisher(publisher: Publisher): Promise<Book[]>;
    getBookByGender(gender: Gender): Promise<Book[]>;
    saveBook(book: Book): Promise<void>;
}