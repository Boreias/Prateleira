import { User } from "../../domain/entities/User";
import { IUserRepository } from "../../domain/irepositories/IUserRepository";


export class UserRepository implements IUserRepository {
    getAllUsers(): Promise<User[]> {
        throw new Error("Method not implemented.");
    }
    getUserById(id: number): Promise<User> {
        throw new Error("Method not implemented.");
    }
    getUserByUsername(username: string): Promise<User[]> {
        throw new Error("Method not implemented.");
    }
    getUserByNickname(nickname: string): Promise<User[]> {
        throw new Error("Method not implemented.");
    }
    saveUser(user: User): Promise<void> {
        throw new Error("Method not implemented.");
    }
}