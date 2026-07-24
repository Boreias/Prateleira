import type { iAuthorRepository } from "@/domain/irepositories/iAuthorRepository";
import { Author } from "@/domain/entities/author";
import api from "../api/api";


export class AuthorRepository implements iAuthorRepository {
    private readonly apiUrl = "author/";

    async createAuthor(authorObject: any): Promise<any> {
        const response = await api.post(`${this.apiUrl}create`, authorObject);

        return response;
    }

    async getAuthorById(id: string): Promise<Author> {
        const response = await api.get(`${this.apiUrl}id?id=${id}`);

        return new Author(
            response.data.id,
            response.data.name,
            response.data.avatar,
            response.data.books
        );
    }

    async getAuthorByName(name: string, skip?: number, page_size?: number): Promise<Author[]> {
        const response = await api.get(`${this.apiUrl}name?name=${name}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((author: any) => new Author(
            author.id,
            author.name,
            author.avatar,
            author.books
        ));
    }

    async getAuthorsByBook(book_id: string, skip?: number, page_size?: number): Promise<Author[]> {
        const response = await api.get(`${this.apiUrl}book?book_id=${book_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((author: any) => new Author(
            author.id,
            author.name,
            author.avatar,
            author.books
        ));
    }

    async getAuthorsByGender(gender_id: string, skip?: number, page_size?: number): Promise<Author[]> {
        const response = await api.get(`${this.apiUrl}gender?gender_id=${gender_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((author: any) => new Author(
            author.id,
            author.name,
            author.avatar,
            author.books
        ));
    }

    async getAuthorsByPublisher(publisher_id: string, skip?: number, page_size?: number): Promise<Author[]> {
        const response = await api.get(`${this.apiUrl}publisher?publisher_id=${publisher_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((author: any) => new Author(
            author.id,
            author.name,
            author.avatar,
            author.books
        ));
    }

    async morePopularAuthor(skip?: number, page_size?: number): Promise<Author[]> {
        const response = await api.get(`${this.apiUrl}more_popular?skip=${skip}&page_size=${page_size}`);

        return response.data.map((author: any) => new Author(
            author.id,
            author.name,
            author.avatar,
            author.books
        ));
    }

    async bestValuatedAuthor(skip?: number, page_size?: number): Promise<Author[]> {
        const response = await api.get(`${this.apiUrl}best_valuated?skip=${skip}&page_size=${page_size}`);

        return response.data.map((author: any) => new Author(
            author.id,
            author.name,
            author.avatar,
            author.books
        ));
    }

    async alterAuthor(id: string, authorObject: any): Promise<any> {
        const response = await api.put(`${this.apiUrl}alter/${id}`, authorObject)

        return response;
    }

    async deleteAuthor(id: string, user_id: string): Promise<any> {
        const response = await api.delete(`${this.apiUrl}delete?id=${id}&user_id=${user_id}`)

        return response;
    }

    async clearDeletedAuthors(): Promise<any> {
        const response = await api.get(`${this.apiUrl}clear_deleted`);

        return response;
    }
}