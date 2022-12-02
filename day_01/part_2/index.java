import java.util.Scanner;

class TopElfes {
  Elf[] elfes;

  TopElfes() {
    this.elfes = new Elf[3];
    this.elfes[0] = new Elf(0, 0);
    this.elfes[1] = new Elf(0, 0);
    this.elfes[2] = new Elf(0, 0);
  }

  void insert(Elf elf) {
    if (elf.calories > this.elfes[2].calories) {
      this.elfes[2] = elf;
    } else {
      return;
    }

    if (this.elfes[2].calories > this.elfes[1].calories) {
      this.swap(2, 1);
    }
    if (this.elfes[1].calories > this.elfes[0].calories) {
      this.swap(1, 0);
    }
  }

  private void swap(int a, int b) {
    Elf temp = this.elfes[a];
    this.elfes[a] = this.elfes[b];
    this.elfes[b] = temp;
  }
}

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

    TopElfes topElfes = new TopElfes();

    long index = 0;

    while (scan.hasNextLine()) {
      String firstCalorie = scan.nextLine();
      System.out.println("first+" + firstCalorie);
      processElf(index, Long.parseLong(firstCalorie), topElfes);
      index += 1;
    }

    System.out.println(topElfes.elfes[0].index + " . " + topElfes.elfes[1].index + " . " + topElfes.elfes[2].index);
    System.out
        .println(topElfes.elfes[0].calories + " . " + topElfes.elfes[1].calories + " . " + topElfes.elfes[2].calories);

    System.out.println(topElfes.elfes[0].calories + topElfes.elfes[1].calories + topElfes.elfes[2].calories);
  }

  public static void processElf(long index, long firstCalorie, TopElfes topElfes) {
    Elf currentElf = new Elf(index, firstCalorie);
    String foodItemCalories = scan.nextLine();
    while (!"".equals(foodItemCalories)) {
      System.out.println(foodItemCalories);
      currentElf.calories += Long.parseLong(foodItemCalories);
      if (!scan.hasNextLine())
        break;
      foodItemCalories = scan.nextLine();
    }
    System.out.println(". " + currentElf.calories);
    topElfes.insert(currentElf);
  }

}