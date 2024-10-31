export const removeUndefinedValues = <T>(obj: object) => {
	return Object.fromEntries(
		Object.entries(obj).filter(([_, value]) => value !== undefined)
	) as T;
};
