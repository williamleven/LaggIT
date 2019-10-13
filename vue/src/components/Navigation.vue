<template>
    <div class="navigation">
        <div class="logo">
            <img src="../assets/laggitlogo.png">
        </div>
        <nav>
            <router-link to="/home">Hem</router-link>
            <router-link to="/events">Evenemang</router-link>
            <router-link to="/about">Om LaggIT</router-link>
            <router-link class="become-member" to="/member">Bli Medlem!</router-link>
        </nav>
    </div>
</template>

<script>
export default {
  name: 'Navigation',
}
</script>

<style lang="scss" scoped>
.navigation {
    width: 100%;
    height: auto;
    background-color: #f8c66e;
    display: flex;
    justify-content: space-between;
    .logo {
        margin: 10px;
        img {
            height: 50px;
        }
    }
    nav {
        display: flex;
        a {
            color: black;
            padding: 15px 35px;
            text-decoration: none;
            display: flex;
            align-items: center;
            &:hover {
                background-color: #fad89d;
            }
        }
        .router-link-active {
            background-color: #fad89d;
        }
        .become-member {
            background-color: #47b5dc;
            &:hover {
                background-color: #6fc9ee;
            }
        }
    }
}
</style>
