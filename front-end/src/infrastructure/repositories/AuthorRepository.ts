import { Author } from "../../domain/entities/Author";
import type { IAuthorRepository } from "../../domain/irepositories/IAuthorRepository";
import api from "../api";


export class AuthorRepository implements IAuthorRepository {
    private readonly apiUrl = "/authors";

    async getAllAuthors(): Promise<Author[]> {
        const response = await api.get(this.apiUrl);
        return response.data.map((author: any) => {
            new Author(
                author._id,
                author._name,
                author._avatar,
                author._books
            )
        })
    }

    async getAuthorById(id: number): Promise<Author> {
        const response = await api.get(`${this.apiUrl}/${id}`);
        return new Author(
            response.data._id,
            response.data._name,
            response.data._avatar,
            response.data._books
        )
    }

    async getAuthorByName(name: string): Promise<Author[]> {
        const response = await api.get(`${this.apiUrl}?name=${name}`);
        return response.data.map((author: any) => {
            new Author(
                author._id,
                author._name,
                author._avatar,
                author._books
            )
        })
    }

    async saveAuthor(author: Author): Promise<void> {
        await api.post(this.apiUrl, author);
    }
}