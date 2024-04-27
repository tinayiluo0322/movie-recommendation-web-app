import pandas as pd

movies = pd.read_csv("combined_data.csv")
movies = movies.dropna(subset=["year"])
subset_movies = movies[movies["year"].isin(["2020", "2021", "2022", "2023"])]
subset_movies.to_csv("combined_data.csv", index=False)
