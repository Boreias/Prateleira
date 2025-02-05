import { Book } from "../../domain/entities/Book";
import { Gender } from "../../domain/entities/Gender";
import { IBookRepository } from "../../domain/irepositories/IBookReposity";


export class BookRepository implements IBookRepository {
    saveBook(book: Book): Promise<void> {
        throw new Error("Method not implemented.");
    }
    async getAllBooks(): Promise<any> {
        throw new Error("Method not implemented.");
    }
    async getBookById(id: number): Promise<any> {
        throw new Error("Method not implemented.");
    }
    async getBookByTitle(title: string): Promise<any> {
        throw new Error("Method not implemented.");
    }
    async getBookByAuthor(author: any): Promise<any> {
        throw new Error("Method not implemented.");
    }
    async getBookByPublisher(publisher: any): Promise<any> {
        throw new Error("Method not implemented.");
    }
    async getBookByGender(gender: Gender): Promise<Book[]> {
        throw new Error("Method not implemented.");
    }
}