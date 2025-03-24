def PascalTriangle(line: int):
    # Caso base
    if line == 1:
        return [1]
    cur = [1, 1]

    # Caso a caso
    for _ in range(2, line):
        for i in range(len(cur) - 1, 0, -1):
            # Somar o números de cima
            cur[i] += cur[i - 1]
        # Adicionar o último número à linha
        cur += [1]
    return cur


def is_number(n):
    # Metodo à pressa de verificar se é um número
    try:
        _ = int(n)
        return True
    except:
        return False


def main():
    print('Digite "ajuda" para obter ajuda.')
    while True:

        i = input(">> ").lower().strip()

        if i == "ajuda" or i == "help":
            print(
                """Comandos:
\t"Ajuda" - Mostra os commandos
\tnúmero (n) - Mostra a linha (n) da piramide de Pascal"""
            )

        elif is_number(i) and int(i) > 0:
            n = int(i)
            line = PascalTriangle(n)
            print(" ".join([str(a) for a in line]))


if __name__ == "__main__":
    main()
