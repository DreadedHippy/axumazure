# AxumAzure
This is a simple Axum web server hosted integrated with Azure services and hosted on Azure and is the final outcome of the ["**Axum meets Azure: Lightning fast web servers**" event](https://twitter.com/HippyDreaded/status/1721917787063406971). The aim is to showcase the feasibility of deploying a Rust backend server to Azure

## Tools used
- Azure Container Registry
- Azure web app for containers
- Azure PostgreSQL flexible server
- Github Actions

## Result
You can view the final result here [https://axumxazuretest.azurewebsites.net](https://axumxazuretest.azurewebsites.net)  <br/>
Routes:
  - GET `/`
  - GET `/test`
  - GET `/api`
  - POST `/api`
  - GET `/api/:id`
  - PATCH `/api/:id`
  - DELETE `/api/:id`

## Recreating
You can deploy this project using the following steps.
1. Fork this repository to your github account
2. Clone the fork to your local machine
3. Create an Azure Container Registry resource and take note of the `login server`, `registry username`, and `registry password`, you should also decide on an `image name` and store it
4. In your fork, go to Settings > Secrets and variables > Actions and add the following repository secrets <br />  AZURE_CONTAINER_REGISTRY_LOGIN_SERVER as `login server` <br /> AZURE_CONTAINER_REGISTRY_USERNAME as `registry username` <br /> AZURE_CONTAINER_REGISTRY_PASSWORD as `registry password` <br /> AZURE_CONTAINER_REGISTRY_IMAGE_NAME as `image name`
5. With these credentials set, make a small change to the code and push to github, your action should run smoothly
6. Create an [Azure Database for PostgreSQL - Flexible server](https://learn.microsoft.com/en-us/azure/postgresql/flexible-server/overview) resource and take note of the `database server`, `username`, `password`. *username and password can be found under Access Keys in the azure portal.
7. Create a **Web App For Container** resource in your azure portal
8. Head to Settings > Configuration > Application Settings > New application settings
9. Head to Deployment Center, select the Container, Repository and tag, and set `Continuous Deployment` to "On"
10. Set the name as `DATABASE_URL` and the value as  `postgresql://{username}:{password}@{database_server}:5432/{database_name: is "postgres" by default}`
11. Make a small change to your code like adding a comment in the file and push to git to see your Action Running and your web server deployed

