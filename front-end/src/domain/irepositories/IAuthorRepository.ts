import { Author } from "../entities/Author";

export interface IAuthorRepository {
    getAllAuthors(): Promise<Author[]>;
    getAuthorById(id: number): Promise<Author>;
    getAuthorByName(name: string): Promise<Author[]>;
    saveAuthor(author: Author): Promise<void>;
}