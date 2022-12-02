import java.util.Scanner;

class Elf {
  long index;
  long calories;

  Elf(long index, long calories) {
    this.index = index;
    this.calories = calories;
  }
}

class Source {
  public static Scanner scan = new Scanner(System.in);

  public static void main(String[] args) {

    Elf maxElf = new Elf(0, 0);
    long index = 0;

    while (scan.hasNextLine()) {
      String firstCalorie = scan.nextLine();
      System.out.println("first+" + firstCalorie);
      maxElf = processElf(index, Long.parseLong(firstCalorie), maxElf);
      index += 1;
      System.out.println("winner: " + maxElf.index);
    }

    System.out.println("max elf index: " + (maxElf.index + 1));
    System.out.println("max elf calories: " + maxElf.calories);
  }

  public static Elf processElf(long index, long firstCalorie, Elf maxElf) {
    long calories = firstCalorie;
    String foodItemCalories = scan.nextLine();
    while (!"".equals(foodItemCalories)) {
      System.out.println(foodItemCalories);
      calories += Long.parseLong(foodItemCalories);
      if (!scan.hasNextLine())
        break;
      foodItemCalories = scan.nextLine();
    }
    System.out.println(". " + calories);
    return calories > maxElf.calories ? new Elf(index, calories) : maxElf;
  }

}