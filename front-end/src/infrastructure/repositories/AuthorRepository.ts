import { Author } from "../../domain/entities/Author";
import { IAuthorRepository } from "../../domain/irepositories/IAuthorRepository";


export class AuthorRepository implements IAuthorRepository {
    getAllAuthors(): Promise<Author[]> {
        throw new Error("Method not implemented.");
    }
    getAuthorById(id: number): Promise<Author> {
        throw new Error("Method not implemented.");
    }
    getAuthorByName(name: string): Promise<Author[]> {
        throw new Error("Method not implemented.");
    }
    saveAuthor(author: Author): Promise<void> {
        throw new Error("Method not implemented.");
    }
}