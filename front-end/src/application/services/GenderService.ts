import type { iGenderRepository } from "@/domain/irepositories/iGenderRepository";
import { Gender } from "@/domain/entities/gender";


export class GenderService {
    constructor(private genderRepository: iGenderRepository) {}

    async createGender(genderObject: any): Promise<any> {
        return await this.genderRepository.createGender(genderObject);
    }

    async getGenderById(id: string): Promise<Gender> {
        return await this.genderRepository.getGenderById(id);
    }

    async getGenderByName(name: string, skip: number, page_size: number): Promise<Gender[]> {
        return await this.genderRepository.getGenderByName(name, skip, page_size);
    }

    async getGendersByBook(book_id: string, skip: number, page_size: number): Promise<Gender[]> {
        return await this.genderRepository.getGendersByBook(book_id, skip, page_size);
    }

    async getGendersByAuthor(author_id: string, skip: number, page_size: number): Promise<Gender[]> {
        return await this.genderRepository.getGendersByAuthor(author_id, skip, page_size);
    }

    async getGendersByPublisher(publisher_id: string, skip: number, page_size: number): Promise<Gender[]> {
        return await this.genderRepository.getGendersByPublisher(publisher_id, skip, page_size);
    }

    async morePopularGender(skip: number, page_size: number): Promise<Gender[]> {
        return await this.genderRepository.morePopularGender(skip, page_size);
    }

    async bestValuatedGender(skip: number, page_size: number): Promise<Gender[]> {
        return await this.genderRepository.bestValuatedGender(skip, page_size);
    }

    async alterGender(id: string, genderObject: any): Promise<any> {
        return await this.genderRepository.alterGender(id, genderObject);
    }

    async deleteGender(id: string, user_id: string): Promise<any> {
        return await this.genderRepository.deleteGender(id, user_id);
    }

    async clearDeletedGenders(): Promise<any> {
        return await this.genderRepository.clearDeletedGenders();
    }
}