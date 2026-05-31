# Perps client

This folder contains a minimal Anchor IDL and TypeScript helper wrappers to call the `perps` program from a React app.

Quick start:

1. Install dependencies for the client library:

```bash
cd contract/client
npm install
```

2. Frontend (FE) app is now located in `contract/FE`. To run the React + Vite frontend with Tailwind:

```bash
cd contract/FE
npm install
npm run dev
```

3. Use the wrapper in your React app (example in `client/src/perps.ts` and `FE/src/src-perps.ts`).

Notes:
- The IDL is generated manually to match the on-chain instruction and account names used by the program.
- After you deploy the program, set the `PROGRAM_ID` in your React app and derive PDAs using the seeds in `programs/perps/src/constants.rs`.
