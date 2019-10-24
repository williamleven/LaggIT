<template>
    <div class="navigation">
        <div class="icons">
            <div class="logo">
                <img src="../assets/laggitlogo.png">
            </div>
            <MenuIcon @click="toggleMenu" class="toggle-menu-icon" title="Öppna Menyn"/>
        </div>
        <nav class="toggle-menu" :class="{'visible': showMenu}">
            <router-link to="/home">Hem</router-link>
            <router-link to="/events">Evenemang</router-link>
            <router-link to="/about">Om LaggIT</router-link>
            <router-link to="/member">Bli Medlem!</router-link>
        </nav>
    </div>
</template>

<script>
import MenuIcon from 'vue-material-design-icons/Menu.vue'

export default {
  name: 'Navigation',
  data () {
    return {
      showMenu: false,
    }
  },
  methods: {
    toggleMenu () {
      this.showMenu = !this.showMenu
    },
  },
  components: {
    MenuIcon,
  },
}
</script>

<style lang="scss" scoped>
.navigation {
    width: 100%;
    height: auto;
    background-color: #47b5dc;
    display: flex;
    justify-content: space-between;
    .logo {
        margin: 10px;
        img {
            height: 30px;
        }
    }
    nav {
        display: flex;
        a {
            color: white;
            padding: 15px 35px;
            text-decoration: none;
            display: flex;
            align-items: center;
            transition: all .5s;
            cursor: pointer;
            &:hover {
                background-color: #6fc9ee;
            }
        }
        .router-link-active {
            background-color: #6fc9ee;
        }
    }
    .toggle-menu-icon {
        display: none;
    }
}
@media only screen and (max-width: 720px) {
    .navigation {
        display: flex;
        flex-direction: column;
        justify-content: initial;
        .icons {
            width: 100%;
            display: flex;
            justify-content: space-between;
            .toggle-menu-icon {
                display: block;
                font-size: 70px;
                align-self: center;
                svg {
                    position: static;
                    color: #176b8a;
                    cursor: pointer;
                    transition: all .5s;
                    &:hover {
                        color: #074055;
                    }
                }
            }
        }
        .toggle-menu {
            display: none;
            a {
                display: flex;
                justify-content: center;
            }
        }
        .visible {
            display: block;
        }
    }
}
</style>
