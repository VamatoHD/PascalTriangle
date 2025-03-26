### Feito por Valério Tomé ###

from pascal import PascalTriangle


def is_number(n):
    try:
        _ = int(n)
        return True
    except:
        return False


ajuda = """Comandos:
\t"Ajuda" - Mostra os commandos
\tnúmero (n) - Mostra a linha (n) do triângulo de Pascal"""


def main():
    print('Digite "ajuda" para obter ajuda.')
    while True:

        i = input(">> ").lower().strip()

        if i == "ajuda" or i == "help":
            print(ajuda)

        elif is_number(i) and int(i) > 0:
            n = int(i)
            line = PascalTriangle(n)
            print(" ".join([str(a) for a in line]))


if __name__ == "__main__":
    main()
