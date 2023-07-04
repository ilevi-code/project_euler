#include <stdio.h>
#include <stdlib.h>

typedef struct num_s
{
    size_t length;
    int* digits;
} num_t;

num_t* num_create(size_t length)
{
    num_t* num = NULL;

    num = malloc(sizeof(num_t*));
    if (num == NULL) {
        return NULL;
    }
    num->digits = calloc(length, sizeof(*num->digits));
    if (num->digits == NULL)
    {
        goto error_free_num;
    }

    num->length = length;
    return num;

error_free_num:
    free(num);
    return NULL;
}

void num_destory(num_t* num)
{
    free(num->digits);
    free(num);
}

int num_enlarge(num_t* num)
{
    int* new_digits = reallocarray(num->digits, num->length + 1, sizeof(*num->digits));
    if (new_digits == NULL) {
        return -1;
    }
    new_digits[num->length] = 0;

    num->digits = new_digits;
    num->length++;

    return 0;
}

void num_multiply(const num_t* num) {
    int remainder = 0;
    int product = 0;
    int digit = 0;

    for (size_t i = 0; i < num->length; i++) {
        product = num->digits[i] * 2;
        digit = product % 10;

        num->digits[i] = digit + remainder;
        remainder = (product - digit) / 10;
    }
}

void num_print(const num_t* num) {
    for (size_t i = 0; i < num->length; i++)
    {
        printf("%d", num->digits[num->length - i - 1]);
    }
    printf("\n");
}

int main() {
    num_t* num = num_create(1);
    if (num == NULL) {
        fprintf(stderr, "Failed to allocate number\n");
        return 1;
    }

    num->digits[0] = 1;

    for (int i = 0; i < 1000; i++)
    {
        if (num->digits[num->length - 1] > 4) {
            if (num_enlarge(num) != 0) {
                fprintf(stderr, "Number enlargment failed\n");
            }
        }
        num_multiply(num);
    }

    int sum = 0;
    for (size_t i = 0; i < num->length; i++)
    {
        sum += num->digits[i];
    }
    printf("sum=%d\n", sum);

    num_destory(num);

    return 0;
}
