import { DisplayError } from "./error";

const uuidv4TestingPattern = /^[0-9A-F]{8}-[0-9A-F]{4}-4[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}$/i;

export function isUuidv4(check: string): boolean {
    return uuidv4TestingPattern.test(check);
}

export function handleDisplayError<F extends (...args: any[]) => any>(f: F): (...args: Parameters<F>) => ReturnType<F> {
    return (...args) => {
        try {
            return f(...args);
        } catch (e) {
            if (e instanceof DisplayError) {
                console.log(e.userFriendlyError);
            }

            throw e;
        }
    }
}