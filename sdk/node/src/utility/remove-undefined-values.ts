export const removeUndefinedValues = <T>(obj: object) => {
	return Object.fromEntries(
		Object.entries(obj).filter(([, value]) => value !== undefined),
	) as T;
};
