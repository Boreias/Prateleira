import { Gender } from "../../domain/entities/Gender";
import { IGenderRepository } from "../../domain/irepositories/IGenderRepository";
import api from "../api";


export class GenderRepository implements IGenderRepository {
    private readonly apiUrl = "/genres";

    async getAllGenders(): Promise<Gender[]> {
        const response = await api.get(this.apiUrl);
        return response.data.map((gender: any) => {
            new Gender(
                gender._id,
                gender._name
            )
        });
    }

    async getGenderById(id: number): Promise<Gender> {
        const response = await api.get(`${this.apiUrl}/${id}`);
        return new Gender(
            response.data._id,
            response.data._name
        )
    }

    async saveGender(gender: Gender): Promise<void> {
        await api.post(this.apiUrl, gender);
    }
}