import { Gender } from "../../domain/entities/Gender";
import { IGenderRepository } from "../../domain/irepositories/IGenderRepository";


export class GenderRepository implements IGenderRepository {
    saveGender(gender: Gender): Promise<void> {
        throw new Error("Method not implemented.");
    }
    getAllGenders(): Promise<any> {
        throw new Error("Method not implemented.");
    }
    getGenderById(id: number): Promise<any> {
        throw new Error("Method not implemented.");
    }
}