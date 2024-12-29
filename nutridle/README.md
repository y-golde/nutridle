1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Launch the Dioxus Fullstack app:

```bash
dx serve --platform fullstack
```

grab the big seed from the fda's website - and run the seeding script, the seeding script will produce nutri_output.json - which is a more lightweight verison of the big FDA file, the server will start by saving the json output file in-memory and providing an api to use it
