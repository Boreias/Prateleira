import { IBookRepository } from "../../domain/irepositories/IBookReposity";
import { Book } from "../../domain/entities/Book";
import { Author } from "../../domain/entities/Author";
import { Publisher } from "../../domain/entities/Publisher";
import { Gender } from "../../domain/entities/Gender";


export class BookService {
    constructor(private BookRepository: IBookRepository){}

    async saveBook(book: Book): Promise<void>{
        await this.BookRepository.saveBook(book);
    }

    async getAllBooks(): Promise<Book[]>{
        return await this.BookRepository.getAllBooks();
    }

    async getBookById(id: number): Promise<Book>{
        return await this.BookRepository.getBookById(id);
    }

    async getBookByTitle(title: string): Promise<Book[]>{
        return await this.BookRepository.getBookByTitle(title);
    }

    async getBookByAuthor(author: Author): Promise<Book[]>{
        return await this.BookRepository.getBookByAuthor(author);
    }

    async getBookByPublisher(publisher: Publisher): Promise<Book[]>{
        return await this.BookRepository.getBookByPublisher(publisher);
    }

    async getBookByGender(gender: Gender): Promise<Book[]>{
        return await this.BookRepository.getBookByGender(gender);
    }
}