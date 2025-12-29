# idropr-sdk-rust
## Secrets
Road map:
- [x] Creating a new api secret
- [ ] Updating api key
- [ ] Updating api secret
- [ ] Updating encryption password (coming soon)
- [ ] Renaming secret (coming soon)
- [ ] Deleting secret (coming soon)

### Create new secret
#### Imports
```ts
import type { CreateSecretBody } from "@battle-texas/sdk/generated/map";
```

#### Body
```ts
const body: CreateSecretBody = {
	name: "battle-houston-api",
	description: "Internal API key for Battle Houston",
	api_key: null,
	api_secret: null
};
```

#### Usage
```ts
await secrets.create(body); // void | throws ApiError
```

# Error Handling
## Imports
```ts
import { ApiError } from "@battle-texas/sdk/error";
```

## Pattern
```ts
try {
	const response = await object.method(dataBody);
} catch (err) {
	if (err instanceof ApiError) {
		console.error("HTTP:", err.httpStatus);
		console.error("API code:", err.apiCode);
		console.error("API reason:", err.apiReason);
		console.error("Message:", err.message);
	} else {
		console.error("Unexpected error:", err);
	}
}
```