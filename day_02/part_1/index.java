import java.util.Scanner;

class Source {
  public static Scanner scan = new Scanner(System.in);

  public static void main(String[] args) {
    long points = 0;
    while (scan.hasNextLine()) {
      String[] moves = scan.nextLine().split(" ");
      System.out.println(inputToMove(moves[0]) + inputToMove(moves[1]));
      points += fightPoints(inputToMove(moves[0]), inputToMove(moves[1]));
      points += movePoints(inputToMove(moves[1]));
    }

    System.out.println(points);

  }

  public static String inputToMove(String input) {
    if ("A".equals(input)) {
      return "R";
    } else if ("B".equals(input)) {
      return "P";
    } else if ("C".equals(input)) {
      return "S";
    } else if ("X".equals(input)) {
      return "R";
    } else if ("Y".equals(input)) {
      return "P";
    } else if ("Z".equals(input)) {
      return "S";
    } else {
      throw new Error("Invalid input");
    }
  }

  public static long movePoints(String move) {
    if ("R".equals(move)) {
      return 1;
    }
    if ("P".equals(move)) {
      return 2;
    }
    return 3;
  }

  public static long fightPoints(String oponent, String player) {

    if (oponent.equals(player)) {
      return 3;
    }

    if ("R".equals(player) && "S".equals(oponent)) {
      return 6;
    }
    if ("S".equals(player) && "P".equals(oponent)) {
      return 6;
    }
    if ("P".equals(player) && "R".equals(oponent)) {
      return 6;
    }

    return 0;
  }

}