/* 
  Converts fahrenheit to celsius
  C = (F -32)/1.8
*/

#include <stdio.h>

int main(void)
{
  double fahrenheit, celsius;

  printf("Enter integer temperature in fahrenheit: ");
  scanf("%lf", &fahrenheit);
  celsius = (fahrenheit -32)/1.8;
  printf("\n %lf fahrenheit is %lf celsius.\n", fahrenheit, celsius);
  return 0;
}