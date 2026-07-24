import type { iAuthorRepository } from "@/domain/irepositories/iAuthorRepository";
import { Author } from "@/domain/entities/author";


export class AuthorService {
    constructor(private authorRepository: iAuthorRepository) {}

    async createAuthor(authorObject: any): Promise<any> {
        return await this.authorRepository.createAuthor(authorObject);
    }

    async getAuthorById(id: string): Promise<Author> {
        return await this.authorRepository.getAuthorById(id);
    }

    async getAuthorByName(name: string, skip?: number, page_size?: number): Promise<Author[]> {
        return await this.authorRepository.getAuthorByName(name, skip, page_size);
    }

    async getAuthorsByBook(book_id: string, skip?: number, page_size?: number): Promise<Author[]> {
        return await this.authorRepository.getAuthorsByBook(book_id, skip, page_size);
    }

    async getAuthorsByGender(gender_id: string, skip?: number, page_size?: number): Promise<Author[]> {
        return await this.authorRepository.getAuthorsByGender(gender_id, skip, page_size);
    }

    async getAuthorsByPublisher(publisher_id: string, skip?: number, page_size?: number): Promise<Author[]> {
        return await this.authorRepository.getAuthorsByPublisher(publisher_id, skip, page_size);
    }

    async morePopularAuthor(skip?: number, page_size?: number): Promise<Author[]> {
        return await this.authorRepository.morePopularAuthor(skip, page_size);
    }

    async bestValuatedAuthor(skip?: number, page_size?: number): Promise<Author[]> {
        return await this.authorRepository.bestValuatedAuthor(skip, page_size);
    }

    async alterAuthor(id: string, authorObject: any): Promise<any> {
        return await this.authorRepository.alterAuthor(id, authorObject);
    }

    async deleteAuthor(id: string, user_id: string): Promise<any> {
        return await this.authorRepository.deleteAuthor(id, user_id);
    }

    async clearDeletedAuthors(): Promise<any> {
        return await this.authorRepository.clearDeletedAuthors();
    }
}