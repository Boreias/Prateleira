import { IUserRepository } from "../../domain/irepositories/IUserRepository";
import { User } from "../../domain/entities/User";


export class UserRepository implements IUserRepository {
    constructor(private UserRepository: IUserRepository) {}

    async getAllUsers(): Promise<User[]> {
        return await this.UserRepository.getAllUsers();
    }

    async getUserById(id: number): Promise<User> {
        return await this.UserRepository.getUserById(id);
    }

    async getUserByUsername(username: string): Promise<User[]> {
        return await this.UserRepository.getUserByUsername(username);
    }

    async getUserByNickname(nickname: string): Promise<User[]> {
        return  await this.UserRepository.getUserByNickname(nickname);
    }

    async saveUser(user: User): Promise<void> {
        await this.UserRepository.saveUser(user);
    }
}