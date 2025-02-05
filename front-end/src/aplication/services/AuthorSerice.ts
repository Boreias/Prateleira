import { IAuthorRepository } from "../../domain/irepositories/IAuthorRepository";
import { Author } from "../../domain/entities/Author";


export class AuthorRepository implements IAuthorRepository {
    constructor(private AuthorRepository: IAuthorRepository) {}

    async getAllAuthors(): Promise<Author[]> {
        return await this.AuthorRepository.getAllAuthors();
    }

    async getAuthorById(id: number): Promise<Author> {
        return await this.AuthorRepository.getAuthorById(id);
    }

    async getAuthorByName(name: string): Promise<Author[]> {
        return await this.AuthorRepository.getAuthorByName(name);
    }

    async saveAuthor(author: Author): Promise<void> {
        await this.AuthorRepository.saveAuthor(author);
    }
}