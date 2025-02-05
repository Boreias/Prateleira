export class Gender {
    constructor(
        private readonly _id: number,
        private _name: string
    ){
        this._id = _id;
        this._name = _name;
    }

    get id(){
        return this._id;
    }

    get name(){
        return this._name;
    }

    setName(name: string){
        this._name = name;
    }
}