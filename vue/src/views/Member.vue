<template>
    <div class="member">
        <section class="left">
            <div>
                <h2>Varför gå med i LaggIT?</h2>
                <p>En motiverande text kring varför man borde bli medlem i LaggIT oavsett vem man är (Chalmerist eller ej, för att LaggIT är typ tvungna via SVEROK att ta in alla typer utav medlemmar även fast alla arr är på Chalmers).
                <br/> <br/> Den här texten kanske borde vara rätt så lång för det finns ju rätt många anledningar till varför man måste gå med i LaggIT, JA, MÅSTE.</p>
            </div>
        </section>
        <section class="right">
            <h2>Bli medlem!</h2>
            <form>
                <div class="input-duo">
                    <div>
                        <label for="first-name">Namn</label>
                        <input type="text" name="first-name" id="first-name">
                    </div>
                    <div>
                        <label for="first-name">Efternamn</label>
                        <input type="text" name="last-name" id="last-name">
                    </div>
                </div>
                <label class="input-distance" for="first-name">Nickname</label>
                <input type="text" name="nick-name" id="nick-name">
                <div class="input-duo input-distance">
                    <div>
                        <label for="first-name">E-mail</label>
                        <input type="text" name="email" id="email">
                    </div>
                    <div>
                        <label for="first-name">Telefonnummer</label>
                        <input type="text" name="telephone" id="telephone">
                    </div>
                </div>
                <label class="input-distance" for="first-name">Adress</label>
                <input type="text" name="address" id="address">
                <div class="input-duo input-distance">
                    <div>
                        <label for="first-name">Postnummer</label>
                        <input type="text" name="postcode" id="postcode">
                    </div>
                    <div>
                        <label for="first-name">Land</label>
                        <input type="text" name="country" id="country">
                    </div>
                </div>
                <button>Bli medlem!</button>
                <!-- Personnummer, adress, telefonnummer, mejl, namn, nick -->
            </form>
        </section>
    </div>
</template>

<script>
export default {
  name: 'Member',
}
</script>

<style lang="scss" scoped>
    .member {
        display: grid;
        grid-template-columns: 45% 55%;
        min-height: 80vh;
        .left {
            grid-column: 1;
            background-color: #074055;
            color: white;
            display: flex;
            justify-content: flex-end;
            align-items: center;
            div {
                display: flex;
                flex-direction: column;
                width: 70%;
                margin-right: 30px;
                h2 {
                    margin: 10px 0;
                }
                p {
                    margin-top: 0;
                    margin-bottom: 20px;
                }
            }
        }
        .right {
            grid-column: 2;
            display: flex;
            justify-content: center;
            flex-direction: column;
            h2 {
                margin-left: 30px;
            }
            form {
                width: 70%;
                display: flex;
                flex-direction: column;
                margin-left: 30px;
                input {
                    height: 21px;
                    padding-left: 2px;
                    font-size: 17px;
                }
                .input-duo {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    grid-gap: 10px;
                    div {
                        display: flex;
                        flex-direction: column;
                    }
                    > div:first-of-type {
                        grid-column: 1;
                    }
                    > div:last-of-type {
                        grid-column: 2;
                    }
                }
                .input-distance {
                    margin-top: 15px;
                }
                button {
                    margin-top: 20px;
                    background-color: #47b5dc;
                    border: none;
                    height: 40px;
                    font-size: 17px;
                    border-radius: 3px;
                    transition: all .5s;
                    cursor: pointer;
                    &:hover {
                        background-color: #6fc9ee;
                    }
                }
            }
        }
    }
</style>
