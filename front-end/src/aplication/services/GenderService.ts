import { IGenderRepository } from "../../domain/irepositories/IGenderRepository";
import { Gender } from "../../domain/entities/Gender";


export class GenderService implements IGenderRepository {
    constructor(private GenderRepository: IGenderRepository) {}

    async getAllGenders(): Promise<Gender[]> {
        return await this.GenderRepository.getAllGenders();
    }

    async getGenderById(id: number): Promise<Gender> {
        return await this.GenderRepository.getGenderById(id);
    }

    async saveGender(gender: Gender): Promise<void> {
        await this.GenderRepository.saveGender(gender);
    }
}