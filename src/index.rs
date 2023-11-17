pub fn index_html() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <style>
* {{
    margin: 0;
    padding: 0;
}}

html, body {{
    height: 100%;
}}

.container {{
    color: white;
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #2d3250;
}}

.subcontainer {{
    font-size: 1.5rem;
    border-radius: 20px;
    width: 80vw;
    height: 60vh;
    background-color: #424769;
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
    align-items: center;
}}

.formulario {{
    width: 100%;
    height: 40vh;
    color: #f9b17a;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;
    font-size: 4rem;
}}

.options {{
    width: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
	flex-wrap: wrap;
	text-align: center;
}}
.option {{
	width: 370px;
}}
.title {{
	text-align: center;
}}

p {{
    text-align: center;
}}

input {{
    text-align: center;
    color: white;
    background-color: transparent;
    border: none;
    border-bottom: 1px solid #676f9d;
    outline: none;
}}

.send {{
    width: 100px;
    height: 30px;
    overflow: hidden;
    border-radius: 5px;
    border: 1px solid black;
    color: black;
    background-color: #f9b17a;
}}
    </style>
</head>
<body>
	<div class="container">
		<div class="subcontainer">
			<div class="title">
				<h1>Pong-Master</h1>
			</div>
			<form class="formulario">
				<div class="options">
					<div class="velocity">
						<p>V</p>
						<input type="text" name="VelocidadValue">
					</div>
					<div class="angle">
						<p>θ</p>
						<input type="text" name="angleValue">
					</div>
				</div>
				<input type="submit" value="Enviar" class="send">
			</form>
		</div>
	</div>
</body>


</html>
        "#
    )
}
