# 🎯 Chat gipity 🎯

A chat for multiple projects

- [Roadmap](https://equatorial-dove-7cd.notion.site/Unnamed-Rust-Project-URP-Roadmap-705a396f297543f69711dd2c53be31c1)



## 💻 Local installation

- Steps

    * In Console build docker images:
  
      `docker-compose -f docker-compose.local.yml build --no-cache --pull`
    * Start all containers:

      `docker-compose -f docker-compose.local.yml up -d`
    * Check health-check:

      `localhost:3000` to check the frontend
    * Setup database
