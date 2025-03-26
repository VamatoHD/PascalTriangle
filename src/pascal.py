### Feito por Valério Tomé ###


def PascalTriangle(line: int) -> list[int]:

    # Caso base: n = 1
    if line == 1:
        return [1]

    # Guarda a linha que se calcula
    # Apenas guarda meio triângulo, aplicando a simetria no final
    cur = [1]

    # Iteramos todas as linhas, começando na n = 3
    for j in range(2, line):
        last = len(cur) - 1
        to_double = cur[last]

        # Adiciona-se os pares da linha anterior
        for i in range(last, 0, -1):
            cur[i] += cur[i - 1]

        # Caso a linha seja impar:
        #   Adiciona-se o número central
        if (j + 1) % 2 != 0:
            cur += [2 * to_double]

    # Aplicar a simetria do triângulo
    if line % 2 == 0:
        return cur + cur[::-1]
    else:
        return cur + cur[-2::-1]
