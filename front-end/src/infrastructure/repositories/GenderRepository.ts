import type { iGenderRepository } from "@/domain/irepositories/iGenderRepository";
import { Gender } from "@/domain/entities/gender";
import api from "../api/api";


export class GenderRepository implements iGenderRepository {
    private readonly apiUrl = "gender/";

    async createGender(genderObject: any): Promise<any> {
        const response = await api.post(`${this.apiUrl}create`, genderObject);

        return response;
    }

    async getGenderById(id: string): Promise<Gender> {
        const response = await api.get(`${this.apiUrl}id?id=${id}`);

        return new Gender(
            response.data.id,
            response.data.name
        );
    }

    async getGenderByName(name: string, skip: number = 0, page_size: number = 20): Promise<Gender[]> {
        const response = await api.get(`${this.apiUrl}name?name=${name}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((gender: any) => new Gender(gender.id, gender.name));
    }

    async getGendersByBook(book_id: string, skip: number = 0, page_size: number = 20): Promise<Gender[]> {
        const response = await api.get(`${this.apiUrl}book?book_id=${book_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((gender: any) => new Gender(gender.id, gender.name));
    }

    async getGendersByAuthor(author_id: string, skip: number = 0, page_size: number = 20): Promise<Gender[]> {
        const response = await api.get(`${this.apiUrl}author?author_id=${author_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((gender: any) => new Gender(gender.id, gender.name));
    }

    async getGendersByPublisher(publisher_id: string, skip: number = 0, page_size: number = 20): Promise<Gender[]> {
        const response = await api.get(`${this.apiUrl}publisher?publisher_id=${publisher_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((gender: any) => new Gender(gender.id, gender.name));
    }

    async morePopularGender(skip: number = 0, page_size: number = 20): Promise<Gender[]> {
        const response = await api.get(`${this.apiUrl}more_popular?skip=${skip}&page_size=${page_size}`);

        return response.data.map((gender: any) => new Gender(gender.id, gender.name));
    }

    async bestValuatedGender(skip: number = 0, page_size: number = 20): Promise<Gender[]> {
        const response = await api.get(`${this.apiUrl}best_valuated?skip=${skip}&page_size=${page_size}`);

        return response.data.map((gender: any) => new Gender(gender.id, gender.name));
    }

    async alterGender(id: string, genderObject: any): Promise<any> {
        const response = await api.put(`${this.apiUrl}alter/${id}`, genderObject)

        return response;
    }

    async deleteGender(id: string, user_id: string): Promise<any> {
        const response = await api.delete(`${this.apiUrl}delete?id=${id}&user_id=${user_id}`)

        return response;
    }

    async clearDeletedGenders(): Promise<any> {
        const response = await api.get(`${this.apiUrl}clear_deleted`);

        return response;
    }
}